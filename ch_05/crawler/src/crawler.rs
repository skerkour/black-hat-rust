use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier},
    time::sleep,
};

pub struct Crawler {
    delay: Duration,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

impl Crawler {
    pub fn new(
        delay: Duration,
        crawling_concurrency: usize,
        processing_concurrency: usize,
    ) -> Self {
        Crawler {
            delay,
            crawling_concurrency,
            processing_concurrency,
        }
    }

    pub async fn run<T: Send + Sync + 'static + std::fmt::Debug>(
        &self,
        spider: Arc<dyn Spider<Item = T>>,
    ) {
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 400;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 10;
        let crawling_delay = self.delay;
        let currently_crawling = Arc::new(AtomicUsize::new(0));

        let (queue_tx, queue_rx) = mpsc::channel(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel(processing_queue_capacity);
        let (results_tx, mut results_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = queue_tx.send(url).await;
        }

        let spider_processor = spider.clone();
        let processor_barrier = barrier.clone();
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(items_rx)
                .for_each_concurrent(processing_concurrency, |item| async {
                    let _ = spider_processor.process(item).await;
                })
                .await;

            processor_barrier.wait().await;
        });

        let spider_crawler = spider.clone();
        let crawler_barrier = barrier.clone();
        let crawling_counter = currently_crawling.clone();
        let crawling_results_tx = results_tx.clone();
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(queue_rx)
                .for_each_concurrent(crawling_concurrency, |queued_url| {
                    let queued_url = queued_url.clone();
                    async {
                        crawling_counter.fetch_add(1, Ordering::SeqCst);
                        let mut urls = Vec::new();
                        let res = spider_crawler
                            .scrap(queued_url.clone())
                            .await
                            .map_err(|err| {
                                log::error!("{}", err);
                                err
                            })
                            .ok();

                        if let Some((items, new_urls)) = res {
                            for item in items {
                                let _ = items_tx.send(item).await;
                            }
                            urls = new_urls;
                        }

                        let _ = crawling_results_tx.send((queued_url, urls)).await;
                        sleep(crawling_delay).await;
                        crawling_counter.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            drop(items_tx);
            crawler_barrier.wait().await;
        });

        loop {
            if let Some((visited_url, new_urls)) = results_rx.try_recv().ok() {
                // let (visited_url, new_urls) = rcv_result.unwrap();
                visited_urls.insert(visited_url);

                for url in &new_urls {
                    if !visited_urls.contains(url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing: {}", url);
                        let _ = queue_tx.send(url.clone()).await;
                    }
                }
            }

            if results_tx.capacity() == crawling_queue_capacity // results channel is empty
            && queue_tx.capacity() == crawling_queue_capacity // queue channel is empty
            && currently_crawling.load(Ordering::SeqCst) == 0
            {
                // no more work, we quit
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        log::info!("crawler: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(queue_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;
    }
}

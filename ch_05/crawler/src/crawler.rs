use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    ops::{AddAssign, SubAssign},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier, Mutex},
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
        let crawling_concurrency = self.crawling_concurrency * 10;
        let crawling_queue_capacity = crawling_concurrency * 10;
        let processing_queue_capacity = self.processing_concurrency;
        let crawling_delay = self.delay;
        let currently_crawling = Arc::new(Mutex::new(0 as usize));

        let (queue_tx, queue_rx) = mpsc::channel(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel(processing_queue_capacity);
        let (results_tx, mut results_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = queue_tx.send(url).await;
        }

        let processing_concurrency = self.processing_concurrency;
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
        let crawler_count = currently_crawling.clone();
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(queue_rx)
                .for_each_concurrent(crawling_concurrency, |queued_url| {
                    let queued_url = queued_url.clone();
                    async {
                        crawler_count.lock().await.add_assign(1);
                        let mut urls = Vec::new();
                        let res = spider_crawler
                            .run(queued_url.clone())
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

                        let _ = results_tx.send((queued_url, urls)).await;
                        sleep(crawling_delay).await;
                        crawler_count.lock().await.sub_assign(1);
                    }
                })
                .await;

            drop(items_tx);
            crawler_barrier.wait().await;
        });

        let mut times_empty = 0;
        loop {
            let rcv_result = results_rx.try_recv();
            if let Some(err) = rcv_result.as_ref().err() {
                if err == &mpsc::error::TryRecvError::Empty
                    && currently_crawling.lock().await.eq(&0)
                {
                    times_empty += 1;
                    if times_empty > 10 {
                        // crawling queue is empty, we quit
                        break;
                    }

                    sleep(crawling_delay).await;
                }

                continue;
            }
            times_empty = 0;

            let (visited_url, new_urls) = rcv_result.unwrap();
            visited_urls.insert(visited_url);

            for url in new_urls {
                if !visited_urls.contains(&url) {
                    visited_urls.insert(url.clone());
                    log::debug!("queueing: {}", &url);
                    let _ = queue_tx.send(url).await;
                }
            }
        }

        log::info!("crawler: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(queue_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;
    }
}

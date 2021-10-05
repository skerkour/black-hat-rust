use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{collections::HashSet, sync::Arc, time::Duration};
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

    pub fn run<T: Send + Sync + 'static + std::fmt::Debug>(
        &self,
        spider: Arc<dyn Spider<Item = T>>,
    ) {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Building tokio's runtime");

        runtime.block_on(async move {
            let mut visited_urls = HashSet::<String>::new();
            let crawling_concurrency = self.crawling_concurrency * 10;
            let crawling_queue_capacity = crawling_concurrency * 10;
            let processing_queue_capacity = self.processing_concurrency * 10;
            let crawling_delay = self.delay;

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
            tokio::spawn(async move {
                tokio_stream::wrappers::ReceiverStream::new(queue_rx)
                    .for_each_concurrent(crawling_concurrency, |queued_url| {
                        let queued_url2 = queued_url.clone();
                        let queued_url3 = queued_url.clone();

                        async {
                            let res = spider_crawler
                                .run(queued_url2)
                                .await
                                .map_err(|err| {
                                    log::error!("{}", err);
                                    err
                                })
                                .ok();

                            let _ = results_tx.send((queued_url3, res)).await;
                            sleep(crawling_delay).await;
                        }
                    })
                    .await;

                crawler_barrier.wait().await;
            });

            let mut times_empty = 0;
            loop {
                let rcv_result = results_rx.try_recv();
                if let Err(err) = rcv_result {
                    match err {
                        mpsc::error::TryRecvError::Empty => {
                            times_empty += 1;
                            if queue_tx.capacity() == crawling_queue_capacity && times_empty > 10 {
                                // crawling queue is empty, we quit
                                break;
                            }

                            sleep(crawling_delay).await;
                            continue;
                        }
                        mpsc::error::TryRecvError::Disconnected => {
                            break;
                        }
                    }
                }

                times_empty = 0;

                let (url, res) = rcv_result.unwrap();

                visited_urls.insert(url);

                if let Some((items, urls)) = res {
                    for item in items {
                        let _ = items_tx.send(item).await;
                    }

                    for url in urls {
                        if !visited_urls.contains(&url) {
                            visited_urls.insert(url.clone());
                            log::info!("queueing: {}", &url);
                            let _ = queue_tx.send(url).await;
                        }
                    }
                }
            }

            log::info!("crawler: control loop exited");

            // we drop the transmitters in order to close the streams
            drop(items_tx);
            drop(queue_tx);

            // and then we wait for the streams to complete
            barrier.wait().await;
        })
    }
}

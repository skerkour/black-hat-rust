use crate::spiders::Spider;
use std::{collections::HashSet, time::Duration};
use tokio::{sync::mpsc, time::sleep};

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

    pub fn run<T>(&self, spider: &dyn Spider<Item = T>) {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Building tokio's runtime");

        runtime.block_on(async move {
            let mut visited_urls = HashSet::<String>::new();
            let (queue_tx, mut queue_rx) = mpsc::unbounded_channel();
            // let (items_tx, mut items_rx) = mpsc::channel(self.processing_concurrency);

            for url in spider.start_urls() {
                visited_urls.insert(url.clone());
                let _ = queue_tx.send(url);
            }

            // tokio::spawn(async move {
            //     tokio_stream::wrappers::UnboundedReceiverStream::new(items_rx)
            //         .for_each_concurrent(self.processing_concurrency, |item| async {
            //             let _ = spider.process(item).await;
            //         })
            //         .await;
            // });

            for queued_url in queue_rx.recv().await {
                let res = spider
                    .run(&queued_url)
                    .await
                    .map_err(|err| {
                        log::error!("{}: {}", &queued_url, err);
                        err
                    })
                    .ok();

                visited_urls.insert(queued_url.clone());

                if let Some((items, urls)) = res {
                    for item in items {
                        let _ = spider.process(item).await;
                        // let _ = items_tx.send(item);
                    }

                    for url in urls {
                        if !visited_urls.contains(&url) {
                            visited_urls.insert(url.clone());
                            log::info!("queueing: {}", &url);
                            let _ = queue_tx.send(url);
                        }
                    }
                }

                sleep(self.delay).await;
            }
        })
    }
}

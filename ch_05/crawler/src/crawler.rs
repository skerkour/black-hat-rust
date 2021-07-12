use crate::spiders::Spider;
use std::{collections::HashSet, time::Duration};
use tokio::time::sleep;

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
            let mut queued_urls = spider.start_urls();

            for queued_url in queued_urls.iter() {
                let res = spider
                    .run(&queued_url)
                    .await
                    .map_err(|err| {
                        log::error!("{}: {}", &queued_url, err);
                        err
                    })
                    .ok();

                visited_urls.insert(queued_url.clone());

                println!("NEXT PAGES: -------------------------------------");
                if let Some((items, urls)) = res {
                    for item in items {
                        let _ = spider.process(item).await;
                    }
                    // TODO: clean urls
                    // for url_to_visit in urls {
                    //     println!("{}", url_to_visit);
                    //     // if !visited_urls.contains(&url_to_visit) {
                    //     //     queued_urls.push(url_to_visit);
                    //     // }
                    // }
                }

                sleep(self.delay).await;
            }
        })
    }
}

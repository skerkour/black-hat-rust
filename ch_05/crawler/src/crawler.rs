use std::{collections::HashSet, time::Duration};

use tokio::time::sleep;

use crate::{processors::Processor, spiders::Spider};

pub struct Crawler {
    delay: Duration,
    concurrency: usize,
}

impl Crawler {
    pub fn new(delay: Duration, concurrency: usize) -> Self {
        Crawler { delay, concurrency }
    }

    pub fn run(&self, spider: Box<dyn Spider>, processor: Box<dyn Processor>) {
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
                if let Some((_, urls)) = res {
                    // TODO: clean urls
                    for url_to_visit in urls {
                        println!("{}", url_to_visit);
                        // if !visited_urls.contains(&url_to_visit) {
                        //     queued_urls.push(url_to_visit);
                        // }
                    }
                }

                sleep(self.delay).await;
            }
        })
    }
}

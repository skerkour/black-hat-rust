use rayon::{ThreadPool, ThreadPoolBuildError};
use reqwest::{blocking::Client, redirect};

use crate::config::{HTTP_TIMEOUT, MAX_REDIRECTS, NUM_THREADS};

pub fn configure_http_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .redirect(redirect::Policy::limited(MAX_REDIRECTS))
        .timeout(HTTP_TIMEOUT)
        .build()
}

pub fn configure_threadpool() -> Result<ThreadPool, ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build()
}

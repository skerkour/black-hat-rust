use crate::{processors::Processor, spiders::Spider};

pub struct Crawler {}

impl Crawler {
    pub fn new() -> Self {
        Crawler {}
    }

    pub fn run(&self, spider: Box<dyn Spider>, processor: Box<dyn Processor>) {}
}

use super::Processor;

pub struct PrintProcessor {}

impl PrintProcessor {
    pub fn new() -> Self {
        PrintProcessor {}
    }
}

impl Processor for PrintProcessor {}

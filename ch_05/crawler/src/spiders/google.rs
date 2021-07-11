pub struct GoogleSpider {}

impl GoogleSpider {
    pub fn new() -> Self {
        GoogleSpider {}
    }
}

impl super::Spider for GoogleSpider {
    fn name(&self) -> String {
        String::from("google")
    }
}

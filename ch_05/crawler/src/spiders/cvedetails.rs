pub struct CveDetailsSpider {}

impl CveDetailsSpider {
    pub fn new() -> Self {
        CveDetailsSpider {}
    }
}

impl super::Spider for CveDetailsSpider {
    fn name(&self) -> String {
        String::from("cvedetails")
    }
}

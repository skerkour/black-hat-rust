pub struct GitHubSpider {}

impl GitHubSpider {
    pub fn new() -> Self {
        GitHubSpider {}
    }
}

impl super::Spider for GitHubSpider {
    fn name(&self) -> String {
        String::from("github")
    }
}

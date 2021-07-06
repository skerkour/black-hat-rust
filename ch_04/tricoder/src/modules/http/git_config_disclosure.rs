use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

pub struct GitConfigDisclosure {
    git_config_regex: Regex,
}

impl GitConfigDisclosure {
    pub fn new() -> Self {
        GitConfigDisclosure {
            git_config_regex: Regex::new(r#"\[branch "[^"]*"\]"#)
                .expect("compiling http/git_config_disclosure regexp"),
        }
    }

    async fn is_git_config_file(&self, content: String) -> Result<bool, Error> {
        let git_config_regex = self.git_config_regex.clone();
        let res = tokio::task::spawn_blocking(move || git_config_regex.is_match(&content)).await?;

        Ok(res)
    }
}

impl Module for GitConfigDisclosure {
    fn name(&self) -> String {
        String::from("http/git_config_disclosure")
    }

    fn description(&self) -> String {
        String::from("Check for .git/config file disclosure")
    }
}

#[async_trait]
impl HttpModule for GitConfigDisclosure {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.git/config", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if self.is_git_config_file(body).await? {
            return Ok(Some(HttpFinding::GitConfigDisclosure(url)));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::GitConfigDisclosure;

    #[tokio::test]
    async fn is_git_directory() {
        let module = GitConfigDisclosure::new();

        let body = r#"[core]
        repositoryformatversion = 0
        filemode = true
        bare = false
        logallrefupdates = true
        ignorecase = true
        precomposeunicode = true
[remote "origin"]
        url = git@github.com:skerkour/black-hat-rust.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
        remote = origin
        merge = refs/heads/master"#;

        let body2 = "test test test test tes  <tle>Index of kerkour.fr</title> test";

        assert_eq!(
            true,
            module.is_git_config_file(body.to_string()).await.unwrap()
        );
        assert_eq!(
            false,
            module.is_git_config_file(body2.to_string()).await.unwrap()
        );
    }
}

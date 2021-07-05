use crate::{model::Subdomain, Error};
use async_trait::async_trait;

mod crtsh;

#[async_trait]
pub trait SubdomainModule {
    async fn enumerate(&self) -> Result<Vec<Subdomain>, Error>;
}

pub use crtsh::*;

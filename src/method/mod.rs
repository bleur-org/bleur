pub mod git;
pub mod http;

use crate::{
    method::{git::Git, http::Http},
    Result,
};
use std::{future::Future, path::PathBuf};
use url::Url;

pub trait Fetchable {
    fn fetch(&self) -> impl Future<Output = Result<()>>;
}

pub trait Methodical {
    fn to_method(&self, url: Url, path: PathBuf) -> Method;
}

#[derive(Debug)]
pub enum Method {
    Git(Git),
    Http(Http),
}

impl Fetchable for Method {
    async fn fetch(&self) -> Result<()> {
        match &self {
            Self::Http(h) => h.fetch().await,
            Self::Git(g) => g.fetch().await,
        }?;

        Ok(())
    }
}

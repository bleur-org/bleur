pub mod git;
pub mod http;

use crate::{
    method::{git::Git, http::Http},
    Result,
};
use std::path::PathBuf;
use url::Url;

pub trait Fetchable {
    fn fetch(&self) -> Result<()>;
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
    fn fetch(&self) -> Result<()> {
        match &self {
            Self::Http(h) => h.fetch(),
            Self::Git(g) => g.fetch(),
        }?;

        Ok(())
    }
}

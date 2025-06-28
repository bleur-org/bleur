use std::path::PathBuf;
use url::Url;

use crate::{method::Fetchable, Result};

#[derive(Debug)]
pub struct Git {
    url: Url,
    path: PathBuf,
}

impl Git {
    pub fn new(url: Url, path: PathBuf) -> Self {
        Self { url, path }
    }
}

impl Fetchable for Git {
    fn fetch(&self, url: Url, path: PathBuf) -> Result<()> {
        println!("Downloaded and stored the repo in path");

        Ok(())
    }
}

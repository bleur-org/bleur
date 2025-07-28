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
    // https://docs.rs/git2/latest/git2/build/struct.RepoBuilder.html
    fn fetch(&self, url: Url, path: PathBuf) -> Result<()> {
        println!("Git mode has been chosen!");
        println!("{url} & {}", path.to_string_lossy());

        Ok(())
    }
}

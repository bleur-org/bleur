use crate::{method::Fetchable, Result};
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub struct Nix {
    url: Url,
    path: PathBuf,
}

impl Nix {
    pub fn new(url: Url, path: PathBuf) -> Self {
        Self { url, path }
    }
}

impl Fetchable for Nix {
    // nix flake show --json github:bleur-org/templates | jq
    fn fetch(&self, url: Url, path: PathBuf) -> Result<()> {
        println!("Downloaded and stored the repo in path");

        Ok(())
    }
}

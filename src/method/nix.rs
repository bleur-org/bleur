use super::scheme::Flake;
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
        println!("Nix mode has been chosen!");
        println!("{url} & {}", path.to_string_lossy());

        let flake = Flake::from_nix_url(&url)?;

        Ok(())
    }
}

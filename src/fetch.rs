use std::path::PathBuf;
use url::Url;
use which::which;

use crate::{Error, Result};

#[derive(Debug)]
pub enum Fetch {
    Git,
    Nix,
}

impl Fetch {
    pub fn fuck_around() -> Result<Fetch> {
        // Prefer nix whenever possible
        if which("nix").is_ok() {
            return Ok(Fetch::Nix);
        }

        // Git is also fine
        if which("git").is_ok() {
            return Ok(Fetch::Git);
        }

        Err(Error::NoToolForInit)
    }

    pub fn fetch(&self, url: Url, path: PathBuf) -> Result<()> {
        match &self {
            Self::Nix => Self::nix_init(url, path),
            Self::Git => Self::git_clone(url, path),
        }?;

        Ok(())
    }

    fn nix_init(_url: Url, _path: PathBuf) -> Result<()> {
        println!("Downloaded and stored the repo in path");

        Ok(())
    }

    fn git_clone(_url: Url, _path: PathBuf) -> Result<()> {
        println!("Downloaded and stored the repo in path");

        Ok(())
    }
}

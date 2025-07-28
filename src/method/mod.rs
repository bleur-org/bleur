mod git;
mod nix;
mod scheme;

use std::path::PathBuf;
use url::Url;
use which::which;

use crate::{
    method::{git::Git, nix::Nix},
    Error, Result,
};

pub trait Fetchable {
    fn fetch(&self, url: Url, path: PathBuf) -> Result<()>;
}

#[derive(Debug)]
pub enum Method {
    Git(Git),
    Nix(Nix),
}

impl Fetchable for Method {
    fn fetch(&self, url: Url, path: PathBuf) -> Result<()> {
        match &self {
            Self::Nix(n) => n.fetch(url, path),
            Self::Git(g) => g.fetch(url, path),
        }?;

        Ok(())
    }
}

impl Method {
    pub fn fuck_around(url: Url, path: PathBuf) -> Result<Method> {
        // Prefer nix whenever possible
        if which("nix").is_ok() {
            return Ok(Method::Nix(Nix::new(url, path)));
        }

        // Git is also fine
        if which("git").is_ok() {
            return Ok(Method::Git(Git::new(url, path)));
        }

        Err(Error::NoToolForInit)
    }
}

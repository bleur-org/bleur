pub mod provider;

use crate::error::{BleurError, Result};
use crate::method::git::provider::Provider;
use crate::method::Fetchable;
use git2::FetchOptions;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub struct Git {
    url: Url,
    path: PathBuf,
}

impl Git {
    pub fn new(url: Url, path: PathBuf) -> Self {
        Self { url, path }
    }

    fn clone(&self) -> Result<()> {
        let mut options = FetchOptions::new();
        options.depth(1);

        let provider = Provider::from_url(self.url.clone())?;
        provider.fetch_repo(options, &self.path).map(|_| ())?;

        std::fs::remove_dir_all(self.path.as_path().join(".git"))
            .map_err(|_| BleurError::CantDeleteGitDirectorty)?;

        Ok(())
    }
}

impl Fetchable for Git {
    // https://docs.rs/git2/latest/git2/build/struct.RepoBuilder.html
    fn fetch(&self) -> Result<()> {
        self.clone()?;

        Ok(())
    }
}

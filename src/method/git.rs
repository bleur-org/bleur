use crate::error::{BleurError, Result};
use git2::{build::RepoBuilder, FetchOptions};
use std::path::PathBuf;
use url::Url;

use crate::method::Fetchable;

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

        RepoBuilder::new()
            .fetch_options(options)
            .clone(self.url.as_str(), self.path.as_path())
            .map(|_| ())
            .map_err(BleurError::CantCloneRepository)?;

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

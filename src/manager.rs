use crate::{fetch::Fetch, Error, Result};
use std::path::Path;
use tempfile::{tempdir, TempDir};
use url::Url;

pub struct Manager {
    remote: Url,
    temporary: TempDir,
    method: Fetch,
}

impl Manager {
    pub fn new(remote: Url, temporary: TempDir, method: Fetch) -> Self {
        Self {
            remote,
            temporary,
            method,
        }
    }

    pub fn path(&self) -> &Path {
        self.temporary.path()
    }
}

#[derive(Default)]
pub struct ManageBuilder {
    remote: Option<Url>,
    temporary: Option<TempDir>,
    method: Option<Fetch>,
}

impl ManageBuilder {
    pub fn new() -> Self {
        Self {
            remote: None,
            temporary: None,
            method: None,
        }
    }

    fn tempdir(&mut self) -> Result<()> {
        self.temporary = Some(tempdir().map_err(Error::TemporaryCantCreate)?);

        Ok(())
    }

    pub fn source(self, url: String) -> Result<Self> {
        let remote = match Url::parse(&url) {
            Ok(l) => Some(l),
            Err(e) => return Err(Error::CantParseUrl(e)),
        };

        Ok(Self {
            remote,
            temporary: self.temporary,
            method: self.method,
        })
    }

    pub fn download(self, method: Option<Fetch>) -> Result<Self> {
        let method = match method {
            Some(f) => f,
            None => Fetch::fuck_around()?,
        };

        Ok(Self {
            method: Some(method),
            remote: self.remote,
            temporary: self.temporary,
        })
    }

    pub fn build(mut self) -> Result<Manager> {
        self.tempdir()?;

        Ok(Manager {
            remote: self.remote.unwrap(),
            temporary: self.temporary.unwrap(),
            method: self.method.unwrap(),
        })
    }
}

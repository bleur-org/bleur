use crate::{
    method::{Fetchable, Method},
    Error, Result,
};
use std::path::Path;
use tempfile::{tempdir, TempDir};
use url::Url;

#[derive(Debug)]
pub struct Manager {
    remote: Url,
    temporary: TempDir,
    method: Method,
}

impl Manager {
    pub fn new(remote: Url, temporary: TempDir, method: Method) -> Self {
        Self {
            remote,
            temporary,
            method,
        }
    }

    pub fn instantiate(&self) -> Result<()> {
        self.method
            .fetch(self.remote.clone(), self.path().to_path_buf())?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        self.temporary.path()
    }
}

#[derive(Default, Debug)]
pub struct ManageBuilder {
    remote: Option<Url>,
    temporary: Option<TempDir>,
    method: Option<Method>,
}

impl ManageBuilder {
    pub fn new() -> Self {
        Self {
            remote: None,
            temporary: None,
            method: None,
        }
    }

    pub fn tempdir(self) -> Result<Self> {
        Ok(Self {
            remote: self.remote,
            temporary: Some(tempdir().map_err(Error::TemporaryCantCreate)?),
            method: self.method,
        })
    }

    pub fn source<T>(self, url: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        Ok(Self {
            temporary: self.temporary,
            method: self.method,
            remote: match Url::parse(url.as_ref()) {
                Ok(l) => Some(l),
                Err(e) => return Err(Error::CantParseUrl(e)),
            },
        })
    }

    pub fn fetch_method(self, method: Option<Method>) -> Result<Self> {
        if self.temporary.is_none() || self.remote.is_none() {
            return Err(Error::InsufficientArgumentsToDecide);
        }

        let destination = self.temporary.unwrap();

        let method = match method {
            Some(f) => f,
            None => Method::fuck_around(
                self.remote.clone().unwrap(),
                destination.path().to_path_buf(),
            )?,
        };

        Ok(Self {
            method: Some(method),
            remote: self.remote,
            temporary: Some(destination),
        })
    }

    pub fn build(self) -> Result<Manager> {
        Ok(Manager {
            remote: self.remote.unwrap(),
            temporary: self.temporary.unwrap(),
            method: self.method.unwrap(),
        })
    }
}

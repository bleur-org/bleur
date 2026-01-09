use std::{collections::HashMap, path::PathBuf};

use crate::{
    method::{Fetchable, Method, Methodical},
    schemes::Configuration,
    Error, Result,
};
use dircpy::CopyBuilder;
use tempfile::{tempdir, TempDir};
use url::Url;

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

    pub fn fetch_method<T>(self, method: T) -> Result<Self>
    where
        T: Methodical,
    {
        if self.temporary.is_none() || self.remote.is_none() {
            return Err(Error::InsufficientArgumentsToDecide);
        }

        let destination = self.temporary.unwrap();

        let method = method.to_method(
            self.remote.clone().unwrap(),
            destination.path().to_path_buf(),
        );

        Ok(Self {
            method: Some(method),
            remote: self.remote,
            temporary: Some(destination),
        })
    }

    pub fn build(self) -> Result<Manager> {
        Ok(Manager::new(
            self.remote.unwrap(),
            self.temporary.unwrap(),
            self.method.unwrap(),
        ))
    }
}

#[derive(Debug)]
pub struct Manager {
    remote: Url,
    temporary: TempDir,
    method: Method,
    template: Configuration,
    globals: HashMap<String, String>,
}

impl Manager {
    pub fn new(remote: Url, temporary: TempDir, method: Method) -> Self {
        Self {
            remote,
            temporary,
            method,
            template: Default::default(),
            globals: HashMap::default(),
        }
    }

    pub fn instantiate(self) -> Result<Self> {
        self.method.fetch()?;

        Ok(Self {
            remote: self.remote,
            temporary: self.temporary,
            method: self.method,
            template: self.template,
            globals: self.globals,
        })
    }

    pub fn parse(self) -> Result<Self> {
        let template = Configuration::surely_template(self.temporary.path().to_path_buf())?;

        Ok(Self {
            template,
            remote: self.remote,
            temporary: self.temporary,
            method: self.method,
            globals: self.globals,
        })
    }

    pub fn evaluate(mut self) -> Result<Self> {
        self.template
            .clone()
            .template()?
            .computable()
            .compute(&mut self.globals)?;

        Ok(self)
    }

    pub fn recursively_copy(self, destination: PathBuf) -> Result<Self> {
        CopyBuilder::new(self.template.clone().template()?.path(), destination)
            .overwrite(true)
            .overwrite_if_newer(true)
            .overwrite_if_size_differs(true)
            .run()?;

        Ok(self)
    }
}

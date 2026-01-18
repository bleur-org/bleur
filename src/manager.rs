use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use crate::{
    method::{Fetchable, Method, Methodical},
    schemes::{template::Template, Configuration},
    Error, Result,
};
use dircpy::CopyBuilder;
use regex::{Regex, RegexBuilder};
use tempfile::{tempdir, TempDir};
use url::Url;

pub static REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"@([a-zA-Z0-9_]+)@").build().unwrap());

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

    pub fn tempdir(mut self) -> Result<Self> {
        self.temporary = Some(tempdir().map_err(Error::TemporaryCantCreate)?);
        Ok(self)
    }

    pub fn source<T: AsRef<str>>(mut self, url: T) -> Result<Self> {
        self.remote = Some(Url::parse(url.as_ref()).map_err(Error::CantParseUrl)?);
        Ok(self)
    }

    pub fn fetch_method<T: Methodical>(mut self, method: T) -> Result<Self> {
        let (Some(remote), Some(ref destination)) = (&self.remote, &self.temporary) else {
            return Err(Error::InsufficientArgumentsToDecide);
        };
        self.method = Some(method.to_method(remote.clone(), destination.path().to_path_buf()));
        Ok(self)
    }

    pub fn build(self) -> Result<Manager> {
        Ok(Manager::new(
            self.remote.unwrap(),
            self.temporary.unwrap(),
            self.method.unwrap(),
            todo!(),
        ))
    }
}

#[derive(Debug)]
pub struct Manager {
    remote: Url,
    temporary: TempDir,
    method: Method,
    template: Template,
    globals: HashMap<String, String>,
}

impl Manager {
    pub fn new(remote: Url, temporary: TempDir, method: Method, template: Template) -> Self {
        Self {
            remote,
            temporary,
            method,
            template,
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
            .computable()
            .compute(&mut self.globals)?;

        Ok(self)
    }

    pub fn recursively_copy(self, destination: PathBuf) -> Result<Self> {
        CopyBuilder::new(self.template.clone().path(), destination)
            .overwrite(true)
            .overwrite_if_newer(true)
            .overwrite_if_size_differs(true)
            .run()?;

        Ok(self)
    }
}

/// For HashMap to implement string search
pub trait Glubtastic {
    fn globs<T>(&self, text: T) -> Vec<String>
    where
        T: AsRef<str>;
}

impl Glubtastic for HashMap<String, String> {
    /// Catch all @variable@ references within a string
    fn globs<T>(&self, text: T) -> Vec<String>
    where
        T: AsRef<str>,
    {
        REGEX
            .captures_iter(text.as_ref())
            .map(|caps| {
                let (_, [input]) = caps.extract();
                input.to_owned()
            })
            .collect::<Vec<String>>()
    }
}

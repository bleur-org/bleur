pub mod collections;
pub mod template;

use crate::schemes::{collections::Collections, template::Template};
use crate::{Error, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collections(Collections),
}

impl Configuration {
    pub fn parse(path: PathBuf) -> Result<Self> {
        let text = fs::read_to_string(path.join("bleur.toml"))?;
        toml::from_str::<Template>(&text)
            .map(|t| Self::Template(t.with_path(path)))
            .or_else(|_| toml::from_str::<Collections>(&text).map(Self::Collections))
            .map_err(|_| Error::NoTemplateConfiguration)
    }

    pub fn surely_template(path: PathBuf) -> Result<Template> {
        match Self::parse(path.clone())? {
            Self::Template(t) => Ok(t),
            Self::Collections(c) => {
                let option = inquire::Select::new(
                    "Choose the template you would like to bootstrap:",
                    c.keys(),
                )
                .prompt()
                .map_err(Error::CantParseUserPrompt)?;
                let option = c.select(option).ok_or(Error::NoSuchTemplateInCollection)?;
                Self::surely_template(option.path(path))
            }
        }
    }

    pub fn template(self) -> Result<Template> {
        match self {
            Self::Template(template) => Ok(template),
            Self::Collections(_) => Err(Error::TemplateIsInvalid),
        }
    }
}

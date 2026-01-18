pub mod collections;
pub mod template;

use crate::schemes::{collections::Collections, template::Template};
use crate::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collections(Collections),

    // If repo doesn't have any configuration
    #[default]
    Empty,
}

impl Configuration {
    pub fn parse(path: &Path) -> Self {
        fs::read_to_string(path.join("bleur.toml"))
            .ok()
            .map(|ref text| {
                toml::from_str::<Template>(text)
                    .map(|t| Configuration::Template(t.with_path(path.to_path_buf())))
                    .or_else(|_| {
                        toml::from_str::<Collections>(text).map(Configuration::Collections)
                    })
                    .unwrap_or(Configuration::Empty)
            })
            .unwrap_or(Configuration::Empty)
    }

    pub fn surely_template(path: PathBuf) -> Result<Self> {
        use Configuration::*;

        match Self::parse(&path) {
            // Template found
            Template(template) => Ok(Template(template)),

            // Config not found
            Empty => Err(Error::NoTemplateConfiguration),

            // Collection not found
            Collections(collections) => {
                let choice = inquire::Select::new(
                    "Choose the template you would like to bootstrap:",
                    collections.keys(),
                )
                .prompt()
                .map_err(Error::CantParseUserPrompt)?;

                let template = collections
                    .select(choice)
                    .ok_or(Error::NoSuchTemplateInCollection)?;

                Self::surely_template(template.path(path))
            }
        }
    }

    pub fn template(self) -> Result<Template> {
        match self {
            Configuration::Template(template) => Ok(template),
            Configuration::Empty => Err(Error::TemplateIsInvalid),
            Configuration::Collections(_) => Err(Error::TemplateIsInvalid),
        }
    }
}

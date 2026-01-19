pub mod collections;
pub mod template;

use crate::schemes::{collections::Collections, template::Template};
use crate::{Error, Result};
use std::fs;
use std::path::PathBuf;

static MAX_COLLECTIONS_DEPTH: u8 = 5;

#[derive(Debug, Clone)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collections(Collections),
}

impl Configuration {
    pub fn parse(path: PathBuf) -> Option<Configuration> {
        let config = Some(path.clone())
            .filter(|p| p.exists())
            .map(|p| p.join("bleur.toml"))
            .filter(|p| p.exists())
            .and_then(|p| fs::read_to_string(p).ok());

        // If there's string inside config file
        if let Some(text) = config {
            // And if it's parsible to Template type
            if let Ok(t) = toml::from_str::<Template>(&text) {
                return Some(Configuration::Template(t.with_path(path)));
            }

            // And if it's parsible to Collection type
            if let Ok(c) = toml::from_str::<Collections>(&text) {
                return Some(Configuration::Collections(c));
            }
        };

        // Nothing's there + invalid config file
        None
    }

    pub fn surely_template(path: PathBuf, depth: u8) -> Result<Self> {
        if depth > MAX_COLLECTIONS_DEPTH {
            return Err(Error::AintNoWayThisDeepCollection(depth));
        }

        match Self::parse(path.clone()) {
            Some(Configuration::Template(t)) => Ok(Configuration::Template(t)),
            Some(Configuration::Collections(c)) => {
                let option = inquire::Select::new(
                    "Choose the template you would like to bootstrap:",
                    c.keys(),
                )
                .prompt()
                .map_err(Error::CantParseUserPrompt)?;

                let option = c.select(option).ok_or(Error::NoSuchTemplateInCollection)?;

                Self::surely_template(option.path(path), depth + 1)
            },
            None => Err(Error::NoTemplateConfiguration)
        }
    }

    pub fn template(self) -> Result<Template> {
        match self {
            Configuration::Template(template) => Ok(template),
            Configuration::Collections(_) => Err(Error::TemplateIsInvalid),
        }
    }
}

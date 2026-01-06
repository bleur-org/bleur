pub mod collection;
pub mod template;

use crate::schemes::{collection::Collections, template::Template};
use crate::{Error, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
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
    pub fn parse(path: PathBuf) -> Self {
        let config = Some(path)
            .filter(|p| p.exists())
            .map(|p| p.join("bleur.toml"))
            .filter(|p| p.exists())
            .and_then(|p| fs::read_to_string(p).ok());

        // If there's string inside config file
        if let Some(text) = config {
            // And if it's parsible to Template type
            if let Ok(t) = toml::from_str::<Template>(&text) {
                return Configuration::Template(t);
            }

            // And if it's parsible to Collection type
            if let Ok(c) = toml::from_str::<Collections>(&text) {
                return Configuration::Collections(c);
            }
        };

        // Nothing's there + invalid config file
        Self::Empty
    }

    pub fn surely_template(path: PathBuf) -> Result<Self> {
        use Configuration::*;

        match Self::parse(path.clone()) {
            Template(t) => Ok(Self::Template(t)),
            Empty => Err(Error::NoTemplateConfiguration),
            Collections(c) => {
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
}

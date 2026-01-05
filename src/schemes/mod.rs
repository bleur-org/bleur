pub mod collection;
pub mod template;

use crate::schemes::{collection::Collections, template::Template};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collection(Collections),

    // If repo doesn't have any configuration
    #[default]
    Empty,
}

impl Configuration {
    pub fn parse(temp: PathBuf) -> Self {
        let config = Some(temp)
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
                return Configuration::Collection(c);
            }
        };

        Self::Empty
    }
}

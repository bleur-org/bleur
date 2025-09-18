pub mod collection;
pub mod template;

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::schemes::{collection::Collections, template::Template};

#[derive(Debug, Default)]
pub enum Configuration {
    // If repo is a single template
    Template(Template),

    // If repo contains collection of templates
    Collection(Collections),

    #[default]
    Empty,
}

impl Configuration {
    pub fn parse(temp: PathBuf) -> Self {
        if !Path::exists(temp.as_path()) {
            return Self::Empty;
        }

        // Proceed to read

        // If anything works out...
        Self::Empty
    }
}

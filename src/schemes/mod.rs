pub mod collection;
pub mod template;

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

pub mod collection;
pub mod template;

use crate::schemes::{collection::Collections, template::Template};

#[derive(Debug)]
pub enum Configuration {
    Template(Template),
    Collection(Collections),
}

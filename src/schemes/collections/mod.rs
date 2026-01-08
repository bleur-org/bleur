pub mod collection;

use collection::Collection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collections {
    templates: HashMap<String, Collection>,
}

impl Collections {
    pub fn keys(&self) -> Vec<String> {
        self.templates.keys().map(|i| i.to_owned()).collect()
    }

    pub fn select(&self, selection: String) -> Option<Collection> {
        self.templates.get(&selection).cloned()
    }
}

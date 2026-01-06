use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    path: String,
    description: String,
    welcome: String,
}

impl Collection {
    pub fn path(&self, base: PathBuf) -> PathBuf {
        let addition = self.path.replace("./", "");

        base.join(addition)
    }
}

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

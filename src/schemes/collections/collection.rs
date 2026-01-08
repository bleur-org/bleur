use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

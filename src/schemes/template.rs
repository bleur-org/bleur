use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    placeholder: String,
    types: String,
    default: String,
    source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    project: Project,
    variables: Vec<Variable>,

    /// Only for runtime use!
    /// For path awareness at recursive copying.
    #[serde(skip)]
    pub path: PathBuf,
}

impl Template {
    pub fn with_path(self, path: PathBuf) -> Self {
        Self {
            project: self.project,
            variables: self.variables,
            path,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

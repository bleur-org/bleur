use crate::{execute::Executor, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

impl Variable {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
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

    pub fn variables(self) -> Vec<Variable> {
        self.variables
    }

    pub fn computable(self) -> Executor {
        Executor::consume(self)
    }
}

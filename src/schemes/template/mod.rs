pub mod project;
pub mod variable;

use crate::execute::Executor;
use project::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use variable::Variable;

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

    pub fn variables(&self) -> &Vec<Variable> {
        &self.variables
    }

    pub fn computable(self) -> Executor {
        Executor::consume(self)
    }
}

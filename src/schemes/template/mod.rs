pub mod project;
pub mod replace;
pub mod variable;

use crate::execute::{
    task::{Task, ToTask},
    Executor,
};
use project::Project;
use replace::Replace;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use variable::Variable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    project: Project,

    #[serde(default)]
    variables: Vec<Variable>,

    #[serde(default)]
    replaces: Vec<Replace>,

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
            replaces: self.replaces,
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

    pub fn to_tasks(self) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::new();

        // Appending variables
        tasks.extend(
            self.variables
                .iter()
                .map(|v| v.to_owned().to_task(&self.path))
                .collect::<Vec<Task>>(),
        );

        // Appending replacements
        tasks.extend(
            self.replaces
                .iter()
                .map(|v| v.to_owned().to_task(&self.path))
                .collect::<Vec<Task>>(),
        );

        // Append other types here...

        // Sort tasks
        tasks.sort();

        // Finally
        tasks
    }
}

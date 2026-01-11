pub mod change;
pub mod r#move;
pub mod project;
pub mod variable;

use crate::execute::{
    task::{Task, ToTask},
    Executor,
};
use change::Change;
use project::Project;
use r#move::Move;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use variable::Variable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    project: Project,

    #[serde(default)]
    variable: Vec<Variable>,

    #[serde(default)]
    change: Vec<Change>,

    #[serde(default)]
    replace: Vec<Move>,

    /// Only for runtime use!
    /// For path awareness at recursive copying.
    #[serde(skip)]
    pub path: PathBuf,
}

impl Template {
    pub fn with_path(self, path: PathBuf) -> Self {
        Self {
            project: self.project,
            variable: self.variable,
            replace: self.replace,
            change: self.change,
            path,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn computable(self) -> Executor {
        Executor::consume(self)
    }

    pub fn to_tasks(self) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::new();

        // Appending variables
        tasks.extend(
            self.variable
                .iter()
                .map(|v| v.to_owned().to_task(&self.path))
                .collect::<Vec<Task>>(),
        );

        // Appending replacements
        tasks.extend(
            self.replace
                .iter()
                .map(|v| v.to_owned().to_task(&self.path))
                .collect::<Vec<Task>>(),
        );

        // Appending changes
        tasks.extend(
            self.change
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

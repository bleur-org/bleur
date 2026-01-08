use crate::{
    execute::task::{Task, ToTask},
    Result,
};
use serde::{Deserialize, Serialize};

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

impl ToTask for Variable {
    fn to_task(self) -> Task {
        Task::Rename(self)
    }
}

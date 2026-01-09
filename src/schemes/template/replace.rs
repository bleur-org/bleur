use crate::{
    execute::task::{Task, ToTask},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Replace {
    from: String,
    to: String,
}

impl Replace {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}

impl ToTask for Replace {
    fn to_task(self) -> Task {
        Task::Move(self)
    }
}

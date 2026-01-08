use crate::schemes::template::variable::Variable;
use crate::Result;
use Task::*;

pub trait ToTask {
    fn to_task(self) -> Task;
}

pub enum Task {
    /// Change content in a file
    Rename(Variable),

    /// Move a file from a place to place
    Move,
}

impl Task {
    pub fn execute(&self) -> Result<()> {
        match self {
            Rename(v) => v.execute(),
            Move => Ok(()),
        }
    }
}

use crate::{
    execute::task::{Task, ToTask},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Replace {
    from: PathBuf,
    to: PathBuf,
}

impl Replace {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        Ok(())
    }
}

impl ToTask for Replace {
    fn to_task(self, path: &Path) -> Task {
        Task::Move(Replace {
            from: path.join(self.from),
            to: path.join(self.to),
        })
    }
}

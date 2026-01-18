use crate::schemes::template::prelude::{change::Change, r#move::Move, variable::Variable};
use crate::Result;
use std::collections::HashMap;
use std::path::Path;

pub trait ToTask {
    fn to_task(self, path: &Path) -> Task;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Task {
    /// Ask for global variables
    Variable(Variable),

    /// Move a file from a place to place
    Change(Change),

    /// Change content in a file
    Move(Move),
}

impl Task {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        match self {
            Self::Variable(v) => v.execute(global),
            Self::Change(c) => c.execute(global),
            Self::Move(m) => m.execute(global),
        }
    }

    /// Ordering whether what to perform after what
    fn index(&self) -> u8 {
        match *self {
            // First get the all variables
            Self::Variable(_) => 1,

            // Then proceed with moving folders from->to desitinations
            Self::Change(_) => 2,

            // Then replace contents inside files
            Self::Move(_) => 3,
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

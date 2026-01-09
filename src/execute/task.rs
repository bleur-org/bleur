use crate::schemes::template::replace::Replace;
use crate::schemes::template::variable::Variable;
use crate::Result;

pub trait ToTask {
    fn to_task(self) -> Task;
}

#[derive(PartialEq, Eq)]
pub enum Task {
    /// Change content in a file
    Rename(Variable),

    /// Move a file from a place to place
    Move(Replace),
}

impl Task {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Rename(v) => v.execute(),
            Self::Move(r) => r.execute(),
        }
    }

    /// Ordering whether what to perform after what
    fn index(&self) -> u8 {
        match *self {
            // First do the value replacings
            Self::Rename(_) => 1,

            // Then proceed with moving folders from->to desitinations
            Self::Move(_) => 2,
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

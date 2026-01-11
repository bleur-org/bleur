use crate::{
    execute::task::{Task, ToTask},
    manager::Glubtastic,
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Move {
    from: PathBuf,
    to: PathBuf,
}

impl Move {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        let to = self
            .to
            .to_str()
            .ok_or(Error::InvalidFilePath(self.to.clone()))?
            .to_owned();

        dbg!(&self.from, &self.to);

        let variables: Vec<(String, Option<&String>)> = global
            .globs(to.clone())
            .iter()
            .map(|m| (m.to_owned(), global.get(m)))
            .collect();

        let mut file_name = to;

        for var in variables {
            if let Some(v) = var.1 {
                file_name = file_name.replace(&format!("@{}@", var.0), v);
                continue;
            }

            return Err(Error::NoSuchVariable(var.0.clone()));
        }

        std::fs::rename(&self.from, file_name).map_err(|e| Error::CantMoveFile(e.to_string()))?;

        Ok(())
    }
}

impl ToTask for Move {
    fn to_task(self, path: &Path) -> Task {
        Task::Move(Move {
            from: path.join(self.from),
            to: path.join(self.to),
        })
    }
}

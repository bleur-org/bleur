use crate::{
    execute::task::{Task, ToTask},
    manager::Glubtastic,
    schemes::template::apply::Apply,
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Move {
    /// Take a file at ...
    from: PathBuf,

    /// And then move it to ...
    to: PathBuf,

    /// Functions to apply on value
    #[serde(default)]
    apply: String,
}

impl Move {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        let to = self
            .to
            .to_str()
            .ok_or(Error::InvalidFilePath(self.to.clone()))?
            .to_owned();

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

        let applications = Apply::parse(self.apply.clone());
        file_name = applications.execute(file_name);

        std::fs::rename(&self.from, file_name).map_err(|e| Error::CantMoveFile(e.to_string()))?;

        Ok(())
    }
}

impl ToTask for Move {
    fn to_task(self, path: &Path) -> Task {
        Task::Move(Move {
            from: path.join(self.from),
            to: path.join(self.to),
            apply: self.apply,
        })
    }
}

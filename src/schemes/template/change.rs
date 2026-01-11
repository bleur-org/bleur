use crate::{
    execute::task::{Task, ToTask},
    manager::Glubtastic,
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Change {
    /// Catch phrase or word to locate
    placeholder: String,

    /// Where the file is located
    source: PathBuf,

    /// Computatble value which might contain global variables
    value: String,
}

impl Change {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        let variables: Vec<(String, Option<&String>)> = global
            .globs(self.value.clone())
            .iter()
            .map(|m| (m.to_owned(), global.get(m)))
            .collect();

        let mut contents = fs::read_to_string(self.source.clone())?;

        for var in variables {
            if let Some(v) = var.1 {
                contents = contents.replace(&self.placeholder, v);
                continue;
            }

            return Err(Error::NoSuchVariable(var.0.clone()));
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.source)?;

        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}

impl ToTask for Change {
    fn to_task(self, path: &Path) -> Task {
        Task::Change(Change {
            placeholder: self.placeholder,
            source: path.join(self.source),
            value: self.value,
        })
    }
}

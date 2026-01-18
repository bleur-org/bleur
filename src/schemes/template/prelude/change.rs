use crate::{
    execute::task::{Task, ToTask},
    manager::Glubtastic,
    schemes::template::apply::Apply,
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

    /// Functions to apply on value
    #[serde(default)]
    apply: String,
}

impl Change {
    pub fn execute(&self, global: &mut HashMap<String, String>) -> Result<()> {
        let variables: Vec<(String, Option<&String>)> = global
            .globs(self.value.clone())
            .iter()
            .map(|m| (m.to_owned(), global.get(m)))
            .collect();

        let mut change = self.value.clone();

        for var in variables {
            if let Some(v) = var.1 {
                change = change.replace(&format!("@{}@", var.0), v);
                continue;
            }

            return Err(Error::NoSuchVariable(var.0.clone()));
        }

        let applications = Apply::parse(self.apply.clone());
        let contents = fs::read_to_string(self.source.clone())?
            .replace(&self.placeholder, &applications.execute(change));

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
            apply: self.apply,
        })
    }
}

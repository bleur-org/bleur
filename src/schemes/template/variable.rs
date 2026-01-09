use crate::{
    execute::task::{Task, ToTask},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Variable {
    placeholder: String,
    message: String,
    types: String,
    default: String,
    source: PathBuf,
}

impl Variable {
    pub fn execute(&self) -> Result<()> {
        let prompt = inquire::Text::new(&self.message)
            .with_default(&self.default)
            .with_placeholder(&self.default)
            .prompt()
            .map_err(Error::CantParseUserPrompt)?;

        let contents = fs::read_to_string(self.source.clone())?.replace(&self.placeholder, &prompt);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.source)?;

        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}

impl ToTask for Variable {
    fn to_task(self, path: &Path) -> Task {
        Task::Rename(Variable {
            placeholder: self.placeholder,
            message: self.message,
            types: self.types,
            default: self.default,
            source: path.join(self.source),
        })
    }
}

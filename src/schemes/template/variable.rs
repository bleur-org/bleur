use crate::{
    execute::task::{Task, ToTask},
    Error, Result,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Variable {
    placeholder: String,
    // message: String,
    types: String,
    default: String,
    source: PathBuf,
}

impl Variable {
    pub fn execute(&self) -> Result<()> {
        // let prompt = inquire::Text::new("Choose the template you would like to bootstrap:")
        //     .with_default(&self.default)
        //     .with_initial_value(&self.default)
        //     .prompt()
        //     .map_err(Error::CantParseUserPrompt)?;

        // let contents = fs::read_to_string(self.source);

        debug!("source is: {:?}", self.source);

        Ok(())
    }
}

impl ToTask for Variable {
    fn to_task(self, path: &Path) -> Task {
        Task::Rename(Variable {
            placeholder: self.placeholder,
            types: self.types,
            default: self.default,
            source: path.join(self.source),
        })
    }
}

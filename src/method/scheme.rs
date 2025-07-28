use crate::error::BleurError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

use crate::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    #[serde(rename = "type")]
    types: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    templates: Option<HashMap<String, Template>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flake {
    data: Data,
    decision: Option<String>,
}

impl Flake {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            decision: None,
        }
    }

    pub fn from_nix_url<T>(url: &T) -> Result<Self>
    where
        T: ToString,
    {
        let output = Command::new("nix")
            .arg("flake")
            .arg("show")
            .arg("--json")
            .arg(url.to_string())
            .output()
            .map_err(|_| BleurError::CommandExecutionFail)?;

        if !output.status.success() {
            println!("{}", String::from_utf8(output.stderr).unwrap());

            return Err(BleurError::CommandExecutionFail);
        }

        let output = String::from_utf8(output.stdout).map_err(BleurError::NixInvalidOutput)?;

        let data = serde_json::from_str::<Data>(&output).map_err(BleurError::CantParseShit);

        Ok(Self::new(data?))
    }

    pub fn decide() -> Result<String> {
        Ok("some".to_string())
    }

    pub fn init<T>(url: &T) -> Result<()>
    where
        T: ToString,
    {
        Ok(())
    }
}

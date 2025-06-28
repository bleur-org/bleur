use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    placeholder: String,
    types: String,
    default: String,
    source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    project: Project,
    variables: Vec<Variable>,
}

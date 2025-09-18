use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    path: String,
    description: String,
    welcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collections {
    templates: HashMap<String, Collection>,
}

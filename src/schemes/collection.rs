use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

// impl Parsable for Collections {
//     fn parse() -> Self {

//       Self {}
//     }
// }

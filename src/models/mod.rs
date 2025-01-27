use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub funds: i32,
    pub resources: i32,
}

impl Unit {
    pub fn new(name: &str, funds: i32, resources: i32) -> Self {
        Self {
            name: name.to_string(),
            funds,
            resources,
        }
    }
}
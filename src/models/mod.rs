use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub funds: i32,
    pub resources: i32,
    pub labor: i32, // 労働力
}

impl Unit {
    pub fn new(name: &str, funds: i32, resources: i32, labor: i32) -> Self {
        Self {
            name: name.to_string(),
            funds,
            resources,
            labor,
        }
    }

    // 資源の消費処理
    pub fn consume_resources(&mut self, amount: i32) {
        self.resources -= amount;
    }

    // 資源の生産処理
    pub fn produce_resources(&mut self, amount: i32) {
        self.resources += amount;
    }
}

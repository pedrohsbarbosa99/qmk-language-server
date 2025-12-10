use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub name: String,
    pub detail: String,
    pub description: String,
    pub doc: String,
}

pub type MacroMap = HashMap<String, Macro>;

pub fn load_macros() -> MacroMap {
    let data = include_str!("../../data/macros.json");
    let macros: Vec<Macro> = serde_json::from_str(data).expect("Failed to parse macros.json");
    
    let mut map = HashMap::new();
    for m in macros {
        map.insert(m.name.clone(), m);
    }
    map
}

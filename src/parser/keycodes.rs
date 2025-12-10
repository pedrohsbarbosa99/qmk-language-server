use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keycode {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub category: String,
}

pub type KeycodeMap = HashMap<String, Keycode>;

pub fn load_keycodes() -> KeycodeMap {
    let data = include_str!("../../data/keycodes.json");
    let keycodes: Vec<Keycode> = serde_json::from_str(data).expect("Failed to parse keycodes.json");
    
    let mut map = HashMap::new();
    for kc in keycodes {
        map.insert(kc.name.clone(), kc);
    }
    map
}

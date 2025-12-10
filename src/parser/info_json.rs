use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardInfo {
    pub layouts: HashMap<String, LayoutMap>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LayoutMap {
    pub layout: Vec<LayoutKey>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LayoutKey {
    pub x: f32,
    pub y: f32,
    // we just need the count, but parsing struct needs to match
}

pub fn find_and_load_info_json(start_path: &Path) -> Option<KeyboardInfo> {
    let mut current = start_path.to_path_buf();
    if current.is_file() {
        current.pop();
    }

    loop {
        let info_path = current.join("info.json");
        if info_path.exists() {
             if let Ok(info) = load_info_json(&info_path) {
                 return Some(info);
             }
        }
        
        let keyboard_json = current.join("keyboard.json");
        if keyboard_json.exists() {
             if let Ok(info) = load_info_json(&keyboard_json) {
                 return Some(info);
             }
        }

        if !current.pop() {
            break;
        }
    }
    None
}

fn load_info_json(path: &Path) -> Result<KeyboardInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let info: KeyboardInfo = serde_json::from_str(&content)?;
    Ok(info)
}

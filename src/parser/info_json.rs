use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]

pub struct KeyboardInfo {
    pub keyboard_name: Option<String>,
    pub manufacturer: Option<String>,
    pub maintainer: Option<String>,
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

pub fn find_info_json_path(start_path: &Path) -> Option<std::path::PathBuf> {
    let mut current = start_path.to_path_buf();
    if current.is_file() {
        current.pop();
    }

    loop {
        let info_path = current.join("info.json");
        if info_path.exists() {
            return Some(info_path);
        }
        
        let keyboard_json = current.join("keyboard.json");
        if keyboard_json.exists() {
            return Some(keyboard_json);
        }

        if !current.pop() {
            break;
        }
    }
    None
}

pub fn find_and_load_info_json(start_path: &Path) -> Option<KeyboardInfo> {
    if let Some(path) = find_info_json_path(start_path) {
        if let Ok(info) = load_info_json(&path) {
            return Some(info);
        }
    }
    None
}

fn load_info_json(path: &Path) -> Result<KeyboardInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let info: KeyboardInfo = serde_json::from_str(&content)?;
    Ok(info)
}

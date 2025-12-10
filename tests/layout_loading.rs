use qmk_lsp::parser::info_json::find_and_load_info_json;
use std::fs::{self, File};
use std::io::Write;

use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_load_layout_info() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let dir_path = std::env::temp_dir().join(format!("test_qmk_lsp_{}", now));
    fs::create_dir_all(&dir_path).unwrap();

    let file_path = dir_path.join("info.json");
    let mut file = File::create(&file_path).unwrap();
    
    let content = r#"{
        "keyboard_name": "MyCoolKeyboard",
        "manufacturer": "Acme Corp",
        "maintainer": "jdoe",
        "layouts": {
            "LAYOUT_60_ansi": {
                "layout": [{"x": 0, "y": 0}, {"x": 1, "y": 0}]
            }
        }
    }"#;
    file.write_all(content.as_bytes()).unwrap();

    // Emulate a keymap file path in the same dir
    let keymap_path = dir_path.join("keymap.c");
    
    let info = find_and_load_info_json(&keymap_path);
    
    // Cleanup
    let _ = fs::remove_dir_all(&dir_path);

    assert!(info.is_some());
    let info = info.unwrap();
    
    assert_eq!(info.keyboard_name, Some("MyCoolKeyboard".to_string()));
    assert_eq!(info.manufacturer, Some("Acme Corp".to_string()));
    assert_eq!(info.maintainer, Some("jdoe".to_string()));
    assert!(info.layouts.contains_key("LAYOUT_60_ansi"));
    assert_eq!(info.layouts["LAYOUT_60_ansi"].layout.len(), 2);
}

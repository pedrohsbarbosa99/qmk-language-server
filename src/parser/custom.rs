use regex::Regex;

#[derive(Debug, Clone)]
pub struct CustomKeycode {
    pub name: String,
}

pub fn parse_custom_keycodes(content: &str) -> Vec<CustomKeycode> {
    let re = Regex::new(r"enum\s+custom_keycodes\s*\{(?P<body>[^}]+)\};").unwrap();
    let mut keycodes = Vec::new();

    if let Some(caps) = re.captures(content) {
        let body = &caps["body"];
        
        // Remove comments line by line
        let clean_lines: Vec<&str> = body.lines().map(|line| {
            if let Some(idx) = line.find("//") {
                &line[..idx]
            } else {
                line
            }
        }).collect();
        let clean_body = clean_lines.join(" ");

        // Split by comma and clean up
        for part in clean_body.split(',') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                // Handle potential assignment like MY_KEY = SAFE_RANGE
                let name = trimmed.split('=').next().unwrap_or(trimmed).trim();
                // Ensure it looks like a keycode (alphanumeric + count > 0)
                 if !name.is_empty() {
                     keycodes.push(CustomKeycode {
                        name: name.to_string(),
                    });
                 }
            }
        }
    }
    keycodes
}


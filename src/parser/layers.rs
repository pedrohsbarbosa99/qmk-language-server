use regex::Regex;

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
}

pub fn parse_layers(content: &str) -> Vec<Layer> {
    let re = Regex::new(r"\[(?P<layer>[A-Z0-9_]+)\]\s*=\s*LAYOUT").unwrap();
    re.captures_iter(content)
        .map(|cap| Layer {
            name: cap["layer"].to_string(),
        })
        .collect()
}


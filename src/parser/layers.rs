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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_layers() {
        let content = "
        [BASE] = LAYOUT(
            KC_A, KC_B
        ),
        [LOWER] = LAYOUT(
            KC_1, KC_2
        )
        ";
        let layers = parse_layers(content);
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].name, "BASE");
        assert_eq!(layers[1].name, "LOWER");
    }
}

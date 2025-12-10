use regex::Regex;

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub macro_name: String,
    pub key_count: usize,
    pub span: std::ops::Range<usize>,
}

pub fn parse_layers(content: &str) -> Vec<Layer> {
    // Regex to find the start of a layer definition: [LAYER_NAME] = LAYOUT_MACRO(
    // We capture the layer name and the macro name.
    let re = Regex::new(r"\[(?P<layer>[a-zA-Z0-9_]+)\]\s*=\s*(?P<macro>[a-zA-Z0-9_]+)\s*\(").unwrap();
    
    let mut layers = Vec::new();

    for cap in re.captures_iter(content) {
        let layer_name = cap["layer"].to_string();
        let macro_name = cap["macro"].to_string();
        let match_start = cap.get(0).unwrap().start();
        
        // Find the position immediately after the opening parenthesis of the macro
        let match_end = cap.get(0).unwrap().end();
        
        // Slice the content starting from after the opening '('
        let remaining = &content[match_end..];
        
        // Count keys by balancing parentheses/braces/brackets and counting commas
        let (key_count, consumed) = count_keys_in_macro(remaining);
        
        let total_end = match_end + consumed;

        layers.push(Layer {
            name: layer_name,
            macro_name,
            key_count,
            span: match_start..total_end,
        });
    }
    
    layers
}

/// Counts the number of comma-separated arguments at the top level of the current scope,
/// stopping at a closing parenthesis ')'.
/// Returns (number of items, bytes consumed)
fn count_keys_in_macro(text: &str) -> (usize, usize) {
    let mut depth = 0;
    let mut key_count = 0;
    let mut has_content = false;
    let mut chars = text.char_indices();
    let mut last_idx = 0;
    
    // If the layout is empty "LAYOUT()"
    // we need to handle that.
    
    while let Some((idx, c)) = chars.next() {
        last_idx = idx;
        match c {
            '(' | '{' | '[' => {
                depth += 1;
                has_content = true;
            }
            ')' | '}' | ']' => {
                if depth == 0 {
                    // We found the closing parenthesis of the LAYOUT macro itself
                    if has_content {
                        key_count += 1;
                    }
                    return (key_count, idx + 1);
                }
                depth -= 1;
            }
            ',' => {
                if depth == 0 {
                    key_count += 1;
                    has_content = false; // reset for next item
                }
            }
            c if !c.is_whitespace() => {
                 has_content = true;
            }
            _ => {}
        }
    }
    
    (key_count, last_idx)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_layers_simple() {
        let content = "
            [0] = LAYOUT_60_ansi(
                KC_ESC, KC_1, KC_2, KC_3
            ),
        ";
        let layers = parse_layers(content);
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].name, "0");
        assert_eq!(layers[0].macro_name, "LAYOUT_60_ansi");
        assert_eq!(layers[0].key_count, 4);
    }

    #[test]
    fn test_parse_layers_nested_macros() {
        let content = "
            [1] = LAYOUT(
                KC_A, LT(1, KC_B), LCTL(KC_C)
            ),
        ";
        let layers = parse_layers(content);
        assert_eq!(layers[0].key_count, 3);
    }

    #[test]
    fn test_parse_layers_complex_nested() {
         let content = "
            [2] = LAYOUT(
                KC_A, 
                MO(1), 
                TD(
                    ACTION_TAP_DANCE_DOUBLE(KC_X, KC_Y)
                ), 
                KC_B
            )
         ";
         // Items: KC_A, MO(1), TD(...), KC_B => 4 items
         let layers = parse_layers(content);
         assert_eq!(layers[0].key_count, 4);
    }

    #[test]
    fn test_parse_layers_empty() {
        let content = "[3] = LAYOUT()";
        let layers = parse_layers(content);
        assert_eq!(layers[0].key_count, 0); // Or 1 if empty string counts as 1? split usually gives 1 empty.
        // My logic: has_content starts false.
        // loop chars: ')' -> depth 0. if has_content { count++ }.
        // If content is just whitespace, has_content false.
        // So count 0.
        // If content is "", loop doesn't run? 
        // Wait, regex finds `LAYOUT(`
        // remaining is `)`
        // loop: `)` -> depth 0. has_content is false. returns 0. Correct.
    }
}

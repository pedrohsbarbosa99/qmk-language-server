use qmk_lsp::parser::layers::parse_layers;
use qmk_lsp::parser::custom::parse_custom_keycodes;

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

#[test]
fn test_parse_custom_keycodes() {
    let content = "
    enum custom_keycodes {
        MY_KEY1,
        MY_KEY2 = SAFE_RANGE,
        // Comment
        MY_KEY3
    };
    ";
    let kcs = parse_custom_keycodes(content);
    assert_eq!(kcs.len(), 3);
    assert_eq!(kcs[0].name, "MY_KEY1");
    assert_eq!(kcs[1].name, "MY_KEY2");
    assert_eq!(kcs[2].name, "MY_KEY3");
}

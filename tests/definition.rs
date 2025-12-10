use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_lsp::lsp_types::*;
use tower_lsp::{LspService, jsonrpc};
use qmk_lsp::server::Backend;
use tower_service::Service;

#[tokio::test]
async fn test_goto_definition() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let dir_path = std::env::temp_dir().join(format!("test_qmk_lsp_def_{}", now));
    fs::create_dir_all(&dir_path).unwrap();

    let info_path = dir_path.join("info.json");
    let mut file = File::create(&info_path).unwrap();
    
    let content = r#"{
    "layouts": {
        "LAYOUT_TEST": {
            "layout": []
        }
    }
}"#;
    file.write_all(content.as_bytes()).unwrap();

    let keymap_path = dir_path.join("keymap.c");
    let keymap_uri = Url::from_file_path(&keymap_path).unwrap();

    // Mock client
    let (mut service, _) = LspService::new(|client| Backend::new(client));
    
    // Initialize
    let init_params = InitializeParams::default();
    let init_req = jsonrpc::Request::build("initialize")
        .params(serde_json::to_value(&init_params).unwrap())
        .id(0)
        .finish();
    let _ = service.call(init_req).await.unwrap();

    // Initialized notification
    let initialized_params = InitializedParams {};
    let initialized_req = jsonrpc::Request::build("initialized")
        .params(serde_json::to_value(&initialized_params).unwrap())
        .finish();
    let _ = service.call(initialized_req).await.unwrap();

    // Mock didOpen
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: keymap_uri.clone(),
            language_id: "c".to_string(),
            version: 1,
            text: "LAYOUT_TEST(KC_A)".to_string(),
        },
    };
    let open_req = jsonrpc::Request::build("textDocument/didOpen")
        .params(serde_json::to_value(&open_params).unwrap())
        .finish();
    
    // Notifications don't return response in this mechanism usually, or return None?
    // LspService returns Ok(None) for notifications?
    let _ = service.call(open_req).await.unwrap();

    // Mock definition request
    let def_params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: keymap_uri },
            position: Position::new(0, 5), 
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };
    
    let def_req = jsonrpc::Request::build("textDocument/definition")
        .params(serde_json::to_value(&def_params).unwrap())
        .id(1)
        .finish();

    let response = service.call(def_req).await.unwrap();
    
    // Parse response
    let resp = response.unwrap();
    println!("Response: {:?}", resp);
    let result: Option<GotoDefinitionResponse> = serde_json::from_value(resp.result().unwrap().clone()).unwrap();
    
    assert!(result.is_some());
    
    if let Some(GotoDefinitionResponse::Scalar(location)) = result {
        let expected_uri = Url::from_file_path(&info_path).unwrap();
        assert_eq!(location.uri, expected_uri);
        assert_eq!(location.range.start.line, 2); 
    } else {
        panic!("Expected Scalar response");
    }

    // Cleanup
    let _ = fs::remove_dir_all(&dir_path);
}

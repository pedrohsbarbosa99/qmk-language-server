use crate::parser::keycodes::{load_keycodes, KeycodeMap};
use crate::parser::macros::{load_macros, MacroMap};
use crate::document::{DocumentStore, DocumentData};
use crate::parser::layers::parse_layers;
use crate::parser::custom::parse_custom_keycodes;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct Backend {
    pub client: Client,
    pub keycodes: KeycodeMap,
    pub macros: MacroMap,
    pub documents: DocumentStore,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            keycodes: load_keycodes(),
            macros: load_macros(),
            documents: DocumentStore::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        eprintln!("Received initialize request");
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "QMK LSP initialized!")
            .await;
        eprintln!("Initialized notification processed");
    }

    async fn shutdown(&self) -> Result<()> {
        eprintln!("Shutdown request");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        eprintln!("Opened document: {}", params.text_document.uri);
        let content = params.text_document.text;
        let layers = parse_layers(&content);
        let custom_keycodes = parse_custom_keycodes(&content);
        
        self.documents.documents.insert(params.text_document.uri, DocumentData {
            content,
            layers,
            custom_keycodes,
        });
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            let content = change.text;
            let layers = parse_layers(&content);
            let custom_keycodes = parse_custom_keycodes(&content);

            self.documents.documents.insert(params.text_document.uri, DocumentData {
                content,
                layers,
                custom_keycodes,
            });
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let mut items = Vec::new();

        // 1. KC_* keycodes from JSON
        for (name, kc) in &self.keycodes {
            items.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(kc.description.clone()),
                documentation: Some(Documentation::String(kc.doc.clone())),
                ..Default::default()
            });
        }

        // 2. Custom Keycodes and Layers from parsed document
        if let Some(doc_data) = self.documents.documents.get(&uri) {
             for layer in &doc_data.layers {
                items.push(CompletionItem {
                    label: layer.name.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some("Layer".to_string()),
                    ..Default::default()
                });
            }
             for custom in &doc_data.custom_keycodes {
                items.push(CompletionItem {
                    label: custom.name.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some("Custom Keycode".to_string()),
                    ..Default::default()
                });
            }
        }

        // 3. Macros from JSON
        for (name, mac) in &self.macros {
            items.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(mac.detail.clone()),
                insert_text: Some(format!("{}(", name)), 
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT), 
                documentation: Some(Documentation::String(mac.doc.clone())),
                ..Default::default()
            });
        }


        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        let doc_entry = match self.documents.documents.get(&uri) {
            Some(d) => d,
            None => return Ok(None),
        };
        let content = &doc_entry.content;

        // Helper to convert UTF-16 column to Byte index
        let lines: Vec<&str> = content.lines().collect();
        if position.line as usize >= lines.len() {
             return Ok(None);
        }
        let line = lines[position.line as usize];

        let mut byte_offset = 0;
        let mut utf16_count = 0;
        for (i, c) in line.char_indices() {
             if utf16_count == position.character as usize {
                 byte_offset = i;
                 break;
             }
             if utf16_count > position.character as usize {
                 // Cursor inside a character (unlikely for simple editing but possible)
                 byte_offset = i;
                 break;
             }
             utf16_count += c.len_utf16();
        }
        // If we reached end of line and matched count
        if utf16_count == position.character as usize && byte_offset == 0 && !line.is_empty() {
             byte_offset = line.len(); // End of line
        }
        
        // Now scan for word boundaries using byte indices
        let mut start_byte = byte_offset;
        let mut end_byte = byte_offset;

        // Scan backwards
        while start_byte > 0 {
            let (prev_char_idx, prev_char) = line[..start_byte].char_indices().next_back().unwrap();
             if !prev_char.is_alphanumeric() && prev_char != '_' {
                break;
            }
            start_byte = prev_char_idx;
        }

        // Scan forwards
        while end_byte < line.len() {
             let (cur_char_idx, cur_char) = line[end_byte..].char_indices().next().unwrap();
             if !cur_char.is_alphanumeric() && cur_char != '_' {
                 break;
             }
             end_byte += cur_char.len_utf8();
        }

        if start_byte >= end_byte {
            return Ok(None);
        }

        let word = &line[start_byte..end_byte];
        self.client.log_message(MessageType::INFO, format!("Hover word: '{}'", word)).await;

        let mut hover_text = String::new();

        // Check KC_* keycodes
        if let Some(kc) = self.keycodes.get(word) {
            hover_text = format!(
                "### {}\n\n**Category:** {}\n\n{}\n\n---\n\n#### Documentation\n{}",
                kc.name, kc.category, kc.description, kc.doc
            );
        } 
        // Check Macros (from JSON)
        else if let Some(mac) = self.macros.get(word) {
            hover_text = format!(
                "### {}\n\n**{}**\n\n{}\n\n---\n\n#### Documentation\n{}",
                mac.name, mac.detail, mac.description, mac.doc
            );
        }
        // Check Custom Keycodes
        else if let Some(custom) = doc_entry.custom_keycodes.iter().find(|c| c.name == word) {
             hover_text = format!("### {} (Custom Keycode)\n\nDefined in this file.", custom.name);
        }
        
        if hover_text.is_empty() {
            return Ok(None);
        }

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_text,
            }),
            range: None,
        }))
    }
}

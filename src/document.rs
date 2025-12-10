use dashmap::DashMap;
use tower_lsp::lsp_types::Url;
use crate::parser::layers::Layer;
use crate::parser::custom::CustomKeycode;

pub struct DocumentData {
    pub content: String,
    pub layers: Vec<Layer>,
    pub custom_keycodes: Vec<CustomKeycode>,
}

pub struct DocumentStore {
    pub documents: DashMap<Url, DocumentData>,
}

impl DocumentStore {
    pub fn new() -> self::DocumentStore {
        DocumentStore {
            documents: DashMap::new(),
        }
    }
}

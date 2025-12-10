use tower_lsp::{LspService, Server};
use qmk_lsp::server::Backend;

#[tokio::main]
async fn main() {
    eprintln!("QMK LSP Server starting...");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    eprintln!("Service created, listening on stdin/stdout");
    Server::new(stdin, stdout, socket).serve(service).await;
    eprintln!("Server shutdown.");
}

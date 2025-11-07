use dpscript::lsp::lsp::TreeSitterLs;
use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let (stdin, stdout) = (stdin(), stdout());
    let (service, socket) = LspService::new(TreeSitterLs::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}

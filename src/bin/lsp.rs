use dpscript::lsp::Backend;
use tower_lsp::Server;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = Backend::service();

    Server::new(stdin, stdout, socket).serve(service).await;
}

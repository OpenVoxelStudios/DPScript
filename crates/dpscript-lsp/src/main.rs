use std::{
    collections::HashMap, ops::ControlFlow, path::PathBuf, pin::Pin, sync::Arc, time::Duration,
};

use async_lsp::{
    ClientSocket, LanguageServer, ResponseError, Result,
    client_monitor::ClientProcessMonitorLayer,
    concurrency::ConcurrencyLayer,
    lsp_types::{
        InitializeParams, InitializeResult, OneOf, SemanticToken, SemanticTokenModifier,
        SemanticTokenType, SemanticTokens, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
        SemanticTokensServerCapabilities, ServerCapabilities, request::SemanticTokensFullRequest,
    },
    panic::CatchUnwindLayer,
    router::Router,
    server::LifecycleLayer,
    tracing::TracingLayer,
};
use dashmap::DashMap;
use dpscript_core::MSourceSpan;
use dpscript_parser::{Literal, tast_from_tokens, tokenize_first};
use dpscript_tokenizer::token::Token;
use tokio::fs;
use tower::ServiceBuilder;
use tracing::{Level, info};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct Server {
    client: ClientSocket,
    tokens: Arc<DashMap<PathBuf, Vec<SemanticToken>>>,
}

struct TickEvent;

impl Server {
    fn new_router(client: ClientSocket) -> Router<Self> {
        let mut router = Router::from_language_server(Self {
            client,
            tokens: Arc::new(DashMap::new()),
        });
        router.event(Self::on_tick);
        router
    }

    fn on_tick(&mut self, _: TickEvent) -> ControlFlow<async_lsp::Result<()>> {
        info!("tick");
        // self.counter += 1;
        ControlFlow::Continue(())
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum TokenTypes {
    NAMESPACE,
    TYPE,
    CLASS,
    ENUM,
    INTERFACE,
    STRUCT,
    TYPE_PARAMETER,
    PARAMETER,
    VARIABLE,
    PROPERTY,
    ENUM_MEMBER,
    EVENT,
    FUNCTION,
    METHOD,
    MACRO,
    KEYWORD,
    MODIFIER,
    COMMENT,
    STRING,
    NUMBER,
    REGEXP,
    OPERATOR,
    DECORATOR,
}

impl TokenTypes {
    pub fn all() -> Vec<Self> {
        vec![
            Self::NAMESPACE,
            Self::TYPE,
            Self::CLASS,
            Self::ENUM,
            Self::INTERFACE,
            Self::STRUCT,
            Self::TYPE_PARAMETER,
            Self::PARAMETER,
            Self::VARIABLE,
            Self::PROPERTY,
            Self::ENUM_MEMBER,
            Self::EVENT,
            Self::FUNCTION,
            Self::METHOD,
            Self::MACRO,
            Self::KEYWORD,
            Self::MODIFIER,
            Self::COMMENT,
            Self::STRING,
            Self::NUMBER,
            Self::REGEXP,
            Self::OPERATOR,
            Self::DECORATOR,
        ]
    }

    pub fn tt(&self) -> SemanticTokenType {
        match self {
            Self::NAMESPACE => SemanticTokenType::NAMESPACE,
            Self::TYPE => SemanticTokenType::TYPE,
            Self::CLASS => SemanticTokenType::CLASS,
            Self::ENUM => SemanticTokenType::ENUM,
            Self::INTERFACE => SemanticTokenType::INTERFACE,
            Self::STRUCT => SemanticTokenType::STRUCT,
            Self::TYPE_PARAMETER => SemanticTokenType::TYPE_PARAMETER,
            Self::PARAMETER => SemanticTokenType::PARAMETER,
            Self::VARIABLE => SemanticTokenType::VARIABLE,
            Self::PROPERTY => SemanticTokenType::PROPERTY,
            Self::ENUM_MEMBER => SemanticTokenType::ENUM_MEMBER,
            Self::EVENT => SemanticTokenType::EVENT,
            Self::FUNCTION => SemanticTokenType::FUNCTION,
            Self::METHOD => SemanticTokenType::METHOD,
            Self::MACRO => SemanticTokenType::MACRO,
            Self::KEYWORD => SemanticTokenType::KEYWORD,
            Self::MODIFIER => SemanticTokenType::MODIFIER,
            Self::COMMENT => SemanticTokenType::COMMENT,
            Self::STRING => SemanticTokenType::STRING,
            Self::NUMBER => SemanticTokenType::NUMBER,
            Self::REGEXP => SemanticTokenType::REGEXP,
            Self::OPERATOR => SemanticTokenType::OPERATOR,
            Self::DECORATOR => SemanticTokenType::DECORATOR,
        }
    }
}

impl LanguageServer for Server {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<Result<()>>;

    fn initialize(
        &mut self,
        params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        eprintln!("Initialize with {params:?}");

        Box::pin(async move {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    semantic_tokens_provider: Some(
                        SemanticTokensServerCapabilities::SemanticTokensOptions(
                            SemanticTokensOptions {
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                                legend: SemanticTokensLegend {
                                    token_types: TokenTypes::all()
                                        .into_iter()
                                        .map(|it| it.tt())
                                        .collect(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ),
                    ),
                    ..ServerCapabilities::default()
                },
                server_info: None,
            })
        })
    }

    fn semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> BoxFuture<'static, Result<Option<SemanticTokensResult>, Self::Error>> {
        let client = self.client.clone();
        let tokens = Arc::clone(&self.tokens);

        Box::pin(async move {
            let name = params.text_document.uri.path();

            let content = fs::read_to_string(params.text_document.uri.to_file_path().unwrap())
                .await
                .unwrap();

            let tokens = tokenize_first(name, &content).unwrap();
            // let tokens = tast_from_tokens(tokens).unwrap();

            let mut line = 0;
            let mut pos = 0;

            let tokens = tokens
                .into_iter()
                .map(|(tkn, span)| {
                    let (s_line, s_pos) = span.position(&content);
                    let delta_line = s_line - line;

                    if line != s_line {
                        pos = 0;
                    }

                    let delta_start = s_pos - pos;

                    line = delta_line;
                    pos = s_pos;

                    match tkn {
                        Token::Keyword(_) => SemanticToken {
                            delta_line: delta_line as u32,
                            delta_start: delta_start as u32,
                            length: span.length() as u32,
                            token_modifiers_bitset: 0,
                            token_type: TokenTypes::KEYWORD as u32,
                        },

                        Token::Punct(_) => SemanticToken {
                            delta_line: delta_line as u32,
                            delta_start: delta_start as u32,
                            length: span.length() as u32,
                            token_modifiers_bitset: 0,
                            token_type: TokenTypes::DECORATOR as u32,
                        },

                        Token::Comparison(_) | Token::Assignment(_) | Token::Operator(_) => {
                            SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_start as u32,
                                length: span.length() as u32,
                                token_modifiers_bitset: 0,
                                token_type: TokenTypes::OPERATOR as u32,
                            }
                        }

                        Token::Literal(it) => match it {
                            Literal::Identifier(_) => SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_start as u32,
                                length: span.length() as u32,
                                token_modifiers_bitset: 0,
                                token_type: TokenTypes::NAMESPACE as u32,
                            },

                            Literal::String(_) => SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_start as u32,
                                length: span.length() as u32,
                                token_modifiers_bitset: 0,
                                token_type: TokenTypes::STRING as u32,
                            },

                            Literal::Int(_)
                            | Literal::Long(_)
                            | Literal::Byte(_)
                            | Literal::Float(_)
                            | Literal::Double(_) => SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_start as u32,
                                length: span.length() as u32,
                                token_modifiers_bitset: 0,
                                token_type: TokenTypes::NUMBER as u32,
                            },
                        },
                    }
                })
                .collect::<Vec<_>>();

            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                data: tokens,
                result_id: Some(name.into()),
            })))
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        tokio::spawn({
            let client = client.clone();
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));

                loop {
                    interval.tick().await;
                    if client.emit(TickEvent).is_err() {
                        break;
                    }
                }
            }
        });

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(Server::new_router(client))
    });

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}

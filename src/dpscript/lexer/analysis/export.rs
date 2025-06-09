use crate::{AddSpan, Export, ImportNode, LexerError, Node, Result, Spanned, Token, TokenCursor};

use super::Analyzer;

impl Analyzer<Export> for Export {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Export>> {
        if item.0 != Token::Export {
            return Ok(None);
        }

        let mut buf = Vec::new();

        while let Some(tkn) = cursor.next() {
            if tkn.0 == Token::Semi {
                break;
            }

            buf.push(tkn);
        }

        let mut module = Vec::new();
        let mut items = Vec::new();
        let mut open = false;
        let mut all = false;

        for (i, (item, span)) in buf.iter().enumerate() {
            match item.clone() {
                Token::Slash | Token::Comma => continue,
                _ => {}
            };

            if item.clone() == Token::Star {
                all = true;
                break;
            }

            if item.clone() == Token::LeftBrace {
                open = true;
                continue;
            }

            if item.clone() == Token::RightBrace {
                open = false;
                continue;
            }

            if open {
                // TODO: Nested import modules (`export a/b/c/{a, b, c/d/e, d/e/{f, g}};`)

                let name = match item.clone() {
                    Token::Ident(id) => id,
                    _ => {
                        return Err(LexerError {
                            src: cursor.source(),
                            at: span.clone(),
                            err: format!("Unexpected token while parsing an export: {}", item),
                        }
                        .into());
                    }
                };

                items.push(ImportNode::Object((name, span.clone())));

                continue;
            }

            if i == buf.len() - 1 && !open {
                let name = match item.clone() {
                    Token::Ident(id) => id,
                    _ => {
                        return Err(LexerError {
                            src: cursor.source(),
                            at: span.clone(),
                            err: format!("Unexpected token while parsing an export: {}", item),
                        }
                        .into());
                    }
                };

                items.push(ImportNode::Object((name, span.clone())));

                break;
            }

            if !open && item.clone() != Token::Slash {
                let name = match item.clone() {
                    Token::Ident(id) => id,
                    _ => {
                        return Err(LexerError {
                            src: cursor.source(),
                            at: span.clone(),
                            err: format!("Unexpected token while parsing an export: {}", item),
                        }
                        .into());
                    }
                };

                module.push((name, span.clone()));

                continue;
            }
        }

        let span = item.1.add(buf.last().unwrap().1);

        Ok(Some(Self {
            all,
            items,
            module,
            span,
        }))
    }
}

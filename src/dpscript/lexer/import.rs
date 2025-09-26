use crate::{
    dpscript::{
        ast::{ident::IdentNode, import::ImportNode, node::Node},
        lexer::{Lexer, Result, err::LexerErr},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_import(&mut self) -> Result<Node> {
        debug!("Attempting to read import...");

        self.push();

        let span = self.start_parse(Token::Import)?;
        let (tokens, last) = self.eat_until(Token::Semi);
        let mut path = Vec::new();
        let mut idx = 0;

        loop {
            if idx >= tokens.len() {
                break;
            }

            let cur = tokens.get(idx).unwrap();

            if idx != tokens.len() - 1 {
                let div = tokens.get(idx + 1).unwrap();

                if div.0 != Token::DoubleColon {
                    return Err(LexerErr::ExpectedButGot {
                        src: self.src(),
                        span: div.1.clone(),
                        expect: Token::DoubleColon,
                        got: div.0.clone(),
                    });
                }
            }

            match cur.0.clone() {
                Token::Ident(id) => path.push(IdentNode {
                    span: cur.1,
                    ident: id,
                }),

                other => {
                    return Err(LexerErr::Unexpected {
                        src: self.src(),
                        span: cur.1,
                        tkn: other,
                    });
                }
            };

            idx += 1;
        }

        self.pop_in_place()?;

        debug!("Successfully read import!");

        Ok(Node::Import(ImportNode {
            span: span.add(last),
            imports: vec![path],
        }))
    }
}

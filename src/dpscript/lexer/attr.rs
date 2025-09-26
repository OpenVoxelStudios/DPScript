use crate::dpscript::{lexer::Lexer, tokenizer::Token};

impl Lexer {
    /// Read an attribute.
    /// THIS WILL NOT PUSH/POP THE STACK!
    // TODO: Return type, parsing contents
    pub fn read_attr(&mut self) -> Option<()> {
        if self.peek(0)?.0 == Token::Hash && self.peek(1)?.0 == Token::LeftBracket {
            self.eat()?;
            self.eat()?;

            let _todo = self.eat_block(Token::LeftBracket, Token::RightBracket);

            // TODO
            None
        } else {
            None
        }
    }
}
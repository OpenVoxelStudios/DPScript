use crate::dpscript::tokenizer::Token;

pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    #[doc(alias = "take")]
    #[doc(alias = "next")]
    pub fn eat(&mut self) -> Option<Token> {
        self.pos += 1;
        self.tokens.get(self.pos - 1).cloned()
    }

    #[doc(alias = "peek_one")]
    pub fn nibble(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    #[doc(alias = "peek")]
    pub fn nibble_ahead(&self, ahead: usize) -> Option<&Token> {
        self.tokens.get(self.pos + ahead)
    }

    /// Eat until the [`is_end`] function returns true (or the stream runs out of tokens),
    /// then return all the eaten tokens.
    #[doc(alias = "take_until")]
    pub fn chomp(&mut self, is_end: impl Fn(&Token) -> bool) -> Vec<Token> {
        let mut buf = Vec::new();

        loop {
            if let Some(token) = self.nibble() {
                if is_end(token) {
                    break;
                }

                buf.push(token.clone());
                self.pos += 1;
            } else {
                break;
            }
        }

        buf
    }
}

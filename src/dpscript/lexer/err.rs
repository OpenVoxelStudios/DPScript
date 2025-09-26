use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use crate::dpscript::tokenizer::Token;

#[derive(Debug, Error, Diagnostic)]
pub enum LexerErr {
    #[error("Expected '{tkn:?}'")]
    #[diagnostic(code(lexer::expected))]
    Expected {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
        tkn: Token,
    },

    #[error("Unexpected token: '{tkn:?}'")]
    #[diagnostic(code(lexer::unexpected))]
    Unexpected {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
        tkn: Token,
    },

    #[error("Expected '{expect:?}', got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_but_got))]
    ExpectedButGot {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
        expect: Token,
        got: Token,
    },

    // Exists so we can differentiate between this and the regular one.
    // This one exists specifically for identifying if a parser even started.
    #[error("Expected '{expect:?}', got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_but_got_s))]
    StartParse {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
        expect: Token,
        got: Token,
    },

    #[error("Unexpected end-of-file")]
    #[diagnostic(code(lexer::eof))]
    EOF {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Attempted to pop an empty stack!")]
    #[diagnostic(code(lexer::stack_pop))]
    StackPop {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Expected any of {expected}")]
    #[diagnostic(code(lexer::expected_any_of))]
    ExpectedAny {
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
        expected: String,
    },
}

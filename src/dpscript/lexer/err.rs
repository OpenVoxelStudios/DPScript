use crate::dpscript::tokenizer::Token;
use flexstr::SharedStr;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum LexerErr {
    #[error("Expected '{tkn:?}'")]
    #[diagnostic(code(lexer::expected))]
    Expected {
        #[label("here")]
        span: SourceSpan,
        tkn: Token,
    },

    #[error("Unexpected token: '{tkn:?}'")]
    #[diagnostic(code(lexer::unexpected))]
    Unexpected {
        #[label("here")]
        span: SourceSpan,
        tkn: Token,
    },

    #[error("Expected '{expect:?}', but got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_but_got))]
    ExpectedButGot {
        #[label("here")]
        span: SourceSpan,
        expect: Token,
        got: Token,
    },

    #[error("Expected a non-negative integer!")]
    #[diagnostic(code(lexer::int_was_negative))]
    IntWasNegative {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Integer was out of bounds (exceeded max value or was below minimum value)!")]
    #[diagnostic(code(lexer::int_max_value))]
    IntMaxVal {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Lexer context was incomplete!")]
    #[diagnostic(code(lexer::incomplete_context))]
    IncompleteContext {
        #[label("here")]
        span: SourceSpan,

        #[help]
        cause: &'static str,
    },

    #[error("Undefined type: '{ty}'")]
    #[diagnostic(code(lexer::unknown_type))]
    UnknownType {
        #[label("here")]
        span: SourceSpan,
        ty: SharedStr,
    },

    // Exists so we can differentiate between this and the regular one.
    // This one exists specifically for identifying if a parser even started.
    #[error("Expected '{expect:?}', but got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_but_got_s))]
    StartParse {
        #[label("here")]
        span: SourceSpan,
        expect: Token,
        got: Token,
    },

    #[error("Could not start parsing without a previous expression!")]
    #[diagnostic(code(lexer::no_last_expr))]
    NoLastExpr {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Unexpected end-of-file")]
    #[diagnostic(code(lexer::eof))]
    EOF {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Attempted to pop an empty stack!")]
    #[diagnostic(code(lexer::stack_pop))]
    StackPop {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Expected any of {expected}, but got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_any_of))]
    ExpectedAny {
        #[label("here")]
        span: SourceSpan,
        expected: String,
        got: Token,
    },

    #[error("Expected any of {expected:?}, but got: '{got:?}'")]
    #[diagnostic(code(lexer::expected_any_token))]
    ExpectedAnyToken {
        #[label("here")]
        span: SourceSpan,
        expected: Vec<Token>,
        got: Token,
    },

    #[error("Expression should have only one value!")]
    #[diagnostic(code(lexer::multiple_values))]
    MultipleValues {
        #[label("here")]
        span: SourceSpan,
    },
}

#[derive(Debug, Error, Diagnostic)]
#[error("Parsing error!")]
#[diagnostic(code(dpscript::lexer))]
pub struct LexerFullErr {
    #[source_code]
    pub source_code: NamedSource<SharedStr>,

    #[related]
    pub err: Vec<LexerErr>,
}

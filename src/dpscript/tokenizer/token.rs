use std::fmt;

use flexstr::SharedStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // =============== LITERALS ===============
    /// n (int)
    Int(i64),

    /// n.n (float)
    Float(f32),

    /// n.n (double)
    Double(f64),

    /// "\\"...\\""
    String(SharedStr),

    /// "true" or "false"
    Bool(bool),

    /// "[ident]"
    Ident(SharedStr),

    // =============== SYMBOLS ===============
    /// "!"
    Exclamation,

    /// "&"
    And,

    /// ":"
    Colon,

    /// "::"
    DoubleColon,

    /// ","
    Comma,

    /// "["
    LeftBracket,

    /// "{"
    LeftBrace,

    /// "("
    LeftParen,

    /// "<"
    LeftAngle,

    /// "]"
    RightBracket,

    /// "}"
    RightBrace,

    /// ")"
    RightParen,

    /// ">"
    RightAngle,

    /// ";"
    Semi,

    /// "="
    Equal,

    /// "."
    Dot,

    /// "-"
    Minus,

    /// "+"
    Plus,

    /// "*"
    Star,

    /// "/"
    Slash,

    /// "%"
    Modulo,

    /// "|"
    Or,

    /// "^"
    Xor,

    /// "#"
    Hash,

    /// "~"
    Tilde,

    // =============== GROUPS ===============
    /// "..."
    Ellipsis,

    /// ".."
    Range,

    // =============== KEYWORDS ===============
    /// "if"
    If,

    /// "in"
    In,

    /// "import"
    Import,

    /// "inline"
    Inline,

    // ========================================
    /// "at"
    At,

    /// "as"
    As,

    // ========================================
    /// "selector"
    Selector,

    // ========================================
    /// "export"
    Export,

    /// "enum"
    Enum,

    /// "else"
    Else,

    // ========================================
    /// "fn"
    Fn,

    /// "for"
    For,

    /// "facade"
    Facade,

    // ========================================
    /// "pos"
    Pos,

    /// "pub"
    Pub,

    // ========================================
    /// "const"
    Const,

    /// "compiler"
    Compiler,

    /// "component"
    Component,

    /// "c" - Short version of "component"
    ComponentShort,

    // ========================================
    /// "let"
    Let,

    /// "return"
    Return,

    /// "ref"
    Ref,

    /// "objective"
    Objective,

    /// "module"
    Module,

    /// "tick"
    Tick,

    /// "init"
    Init,

    /// "nbt"
    Nbt,

    /// "while"
    While,

    /// "operator"
    Operator,

    // =============== SPECIAL ===============
    /// "<none>"
    /// This should never be parsed, it should only be used for error messages.
    None,

    /// "EOF"
    /// This should never be parsed, it should only be used for error messages.
    EOF,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Int(_) => write!(f, "int"),
            Self::Float(_) => write!(f, "float"),
            Self::Double(_) => write!(f, "double"),
            Self::String(_) => write!(f, "string"),
            Self::Bool(_) => write!(f, "bool"),
            Self::Ident(_) => write!(f, "ident"),
            Self::Exclamation => write!(f, "!"),
            Self::And => write!(f, "&"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::LeftBracket => write!(f, "["),
            Self::LeftBrace => write!(f, "{{"),
            Self::LeftParen => write!(f, "("),
            Self::LeftAngle => write!(f, "<"),
            Self::RightBracket => write!(f, "]"),
            Self::RightBrace => write!(f, "}}"),
            Self::RightParen => write!(f, ")"),
            Self::RightAngle => write!(f, ">"),
            Self::Semi => write!(f, ";"),
            Self::Equal => write!(f, "="),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Hash => write!(f, "#"),
            Self::Ellipsis => write!(f, "..."),
            Self::Range => write!(f, ".."),
            Self::If => write!(f, "if"),
            Self::In => write!(f, "in"),
            Self::Import => write!(f, "import"),
            Self::Inline => write!(f, "inline"),
            Self::Selector => write!(f, "selector"),
            Self::Export => write!(f, "export"),
            Self::Enum => write!(f, "enum"),
            Self::Else => write!(f, "else"),
            Self::Fn => write!(f, "fn"),
            Self::For => write!(f, "for"),
            Self::Facade => write!(f, "facade"),
            Self::Pub => write!(f, "pub"),
            Self::Const => write!(f, "const"),
            Self::Compiler => write!(f, "compiler"),
            Self::Component => write!(f, "component"),
            Self::ComponentShort => write!(f, "c"),
            Self::Let => write!(f, "let"),
            Self::Return => write!(f, "return"),
            Self::Objective => write!(f, "objective"),
            Self::Module => write!(f, "module"),
            Self::Tick => write!(f, "tick"),
            Self::Init => write!(f, "init"),
            Self::Nbt => write!(f, "nbt"),
            Self::Pos => write!(f, "pos"),
            Self::Tilde => write!(f, "~"),
            Self::Ref => write!(f, "ref"),
            Self::DoubleColon => write!(f, "::"),
            Self::None => write!(f, "<none>"),
            Self::EOF => write!(f, "EOF"),
            Self::Modulo => write!(f, "%"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
            Self::At => write!(f, "at"),
            Self::As => write!(f, "as"),
            Self::While => write!(f, "while"),
            Self::Operator => write!(f, "operator"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(v) => write!(f, "{}f", v),
            Self::Double(v) => write!(f, "{}d", v),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Ident(i) => write!(f, "{}", i),
            Self::Exclamation => write!(f, "!"),
            Self::And => write!(f, "&"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::LeftBracket => write!(f, "["),
            Self::LeftBrace => write!(f, "{{\n"),
            Self::LeftParen => write!(f, "("),
            Self::LeftAngle => write!(f, "<"),
            Self::RightBracket => write!(f, "]"),
            Self::RightBrace => write!(f, "\n}}\n"),
            Self::RightParen => write!(f, ")"),
            Self::RightAngle => write!(f, ">"),
            Self::Semi => write!(f, ";\n"),
            Self::Equal => write!(f, "="),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Hash => write!(f, "#"),
            Self::Ellipsis => write!(f, "..."),
            Self::Range => write!(f, ".."),
            Self::If => write!(f, "if"),
            Self::In => write!(f, "in"),
            Self::Import => write!(f, "import"),
            Self::Inline => write!(f, "inline"),
            Self::Selector => write!(f, "selector"),
            Self::Export => write!(f, "export"),
            Self::Enum => write!(f, "enum"),
            Self::Else => write!(f, "else"),
            Self::Fn => write!(f, "fn"),
            Self::For => write!(f, "for"),
            Self::Facade => write!(f, "facade"),
            Self::Pub => write!(f, "pub"),
            Self::Const => write!(f, "const"),
            Self::Compiler => write!(f, "compiler"),
            Self::Component => write!(f, "component"),
            Self::ComponentShort => write!(f, "c"),
            Self::Let => write!(f, "let"),
            Self::Return => write!(f, "return"),
            Self::Objective => write!(f, "objective"),
            Self::Module => write!(f, "module"),
            Self::Tick => write!(f, "tick"),
            Self::Init => write!(f, "init"),
            Self::Nbt => write!(f, "nbt"),
            Self::Pos => write!(f, "pos"),
            Self::Tilde => write!(f, "~"),
            Self::Ref => write!(f, "ref"),
            Self::DoubleColon => write!(f, "::"),
            Self::None => write!(f, "<none>"),
            Self::EOF => write!(f, "EOF"),
            Self::Modulo => write!(f, "%"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
            Self::At => write!(f, "at"),
            Self::As => write!(f, "as"),
            Self::While => write!(f, "while"),
            Self::Operator => write!(f, "operator"),
        }
    }
}

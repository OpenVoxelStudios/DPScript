mod assign;
mod attr;
mod block;
mod call;
mod cond;
mod enums;
mod export;
mod func;
mod ident;
mod import;
mod literal;
mod loops;
mod module;
mod node;
mod objective;
mod ops;
mod ret;
mod sub;
mod ty;
mod var;

use super::TokenCursor;
use crate::{Node, Result, Spanned, Token};

pub trait Analyzer<T> {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<T>>;
}

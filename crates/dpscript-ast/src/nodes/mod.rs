pub mod def;
pub mod err;
pub mod expr;
pub mod meta;
pub mod types;
pub mod value;

mod util;

util::node_group! {
    Node = [
        def::Def,
        expr::Expr,
        value::Value
    ]
}

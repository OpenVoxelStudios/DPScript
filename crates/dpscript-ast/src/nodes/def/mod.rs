pub mod constant;
pub mod enums;
pub mod func;
pub mod import;
pub mod objective;
pub mod structs;
pub mod block;

crate::nodes::util::node_group! {
    Def = [
        constant::Constant,
        enums::Enum,
        func::Function,
        import::Import,
        objective::Objective,
        structs::Struct,
        block::Block,
    ]
}

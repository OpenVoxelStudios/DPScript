use miette::SourceSpan;

use crate::dpscript::{
    ast::{
        ast::Scope, binop::BinaryOpNode, block::BlockNode, constant::ConstantNode,
        func::FunctionNode, literal::LiteralNode, unop::UnaryOpNode, var::VarNode,
    },
    data::NodeInfo,
    ty::TypeRef,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Node {
    pub span: SourceSpan,
    pub data: NodeData,

    /// Whether the node is an "ending" node, which means it didn't have a semicolon.
    /// If the node returns a value, then it will be returned from whatever block it is in (if any).
    pub is_end: bool,
}

macro_rules! node_data {
    { $($variant: ident: $data: ty,)* } => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub enum NodeData {
            $($variant($data),)*
        }

        impl NodeInfo for Node {
            fn is_const(&self, scope: &Scope) -> bool {
                match &self.data {
                    $(NodeData::$variant(me) => me.is_const(scope),)*
                }
            }

            fn returns(&self, scope: &Scope) -> Option<TypeRef> {
                match &self.data {
                    $(NodeData::$variant(me) => me.returns(scope),)*
                }
            }
        }
    };
}

node_data! {
    Constant: ConstantNode,
    Function: FunctionNode,
    UnaryOp: UnaryOpNode,
    BinaryOp: BinaryOpNode,
    Variable: VarNode,
    Block: BlockNode,
    Literal: LiteralNode,
}

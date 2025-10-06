use crate::dpscript::{
    ast::{
        ast::Scope, binop::BinaryOpNode, block::BlockNode, call::CallNode, cond::ConditionalNode,
        constant::ConstantNode, enums::EnumNode, func::FunctionNode, ident::IdentNode,
        import::ImportNode, literal::LiteralNode, loops::LoopNode, objective::ObjectiveNode,
        ret::ReturnNode, special::SpecialNode, unop::UnaryOpNode, var::VarNode,
    },
    data::NodeInfo,
    ty::TypeRef,
};

macro_rules! node_data {
    { $($variant: ident: $data: ty,)* } => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpanGroup)]
        pub enum Node {
            $($variant($data),)*
        }

        impl NodeInfo for Node {
            fn is_const(&self, scope: &Scope) -> bool {
                match self {
                    $(Node::$variant(me) => me.is_const(scope),)*
                }
            }

            fn returns(&self, scope: &Scope) -> Option<TypeRef> {
                match self {
                    $(Node::$variant(me) => me.returns(scope),)*
                }
            }
        }

        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Node::$variant(me) => write!(f, "{me}"),)*
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
    Call: CallNode,
    Conditional: ConditionalNode,
    Enum: EnumNode,
    Ident: IdentNode,
    Loop: LoopNode,
    Objective: ObjectiveNode,
    Import: ImportNode,
    Return: ReturnNode,
    Special: SpecialNode,
}

impl Node {
    pub fn maybe_has_value(&self) -> bool {
        match self {
            Node::Constant(_)
            | Node::Function(_)
            | Node::Variable(_)
            | Node::Block(_)
            | Node::Conditional(_)
            | Node::Enum(_)
            | Node::Loop(_)
            | Node::Objective(_)
            | Node::Import(_)
            | Node::Return(_) => false,

            Node::UnaryOp(_)
            | Node::BinaryOp(_)
            | Node::Literal(_)
            | Node::Call(_)
            | Node::Ident(_)
            | Node::Special(_) => true,
        }
    }
}

use crate::dpscript::{
    ast::{
        ast::Scope, at::AtNode, binop::BinaryOpNode, block::BlockNode, call::CallNode,
        cond::ConditionalNode, constant::ConstantNode, enums::EnumNode, func::FunctionNode,
        ident::IdentNode, import::ImportNode, literal::LiteralNode, loops::LoopNode,
        objective::ObjectiveNode, ret::ReturnNode, special::SpecialNode, unop::UnaryOpNode,
        var::VarNode,
    },
    data::NodeInfo,
    ty::TypeRef,
};

macro_rules! node_data {
    { $($id: ident = $variant: ident: $data: ty,)* } => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpanGroup)]
        pub enum Node {
            $($variant($data),)*
        }

        $(
            concat_idents::concat_idents!(name = is_, $id {
                impl Node {
                    pub fn name(&self) -> bool {
                        match self {
                            Self::$variant(_) => true,
                            _ => false,
                        }
                    }
                }
            });

            concat_idents::concat_idents!(name = as_, $id {
                impl Node {
                    pub fn name(self) -> Option<$data> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }
                }
            });
        )*

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
    constant = Constant: ConstantNode,
    function = Function: FunctionNode,
    unary_op = UnaryOp: UnaryOpNode,
    binary_op = BinaryOp: BinaryOpNode,
    variable = Variable: VarNode,
    block = Block: BlockNode,
    literal = Literal: LiteralNode,
    call = Call: CallNode,
    conditional = Conditional: ConditionalNode,
    r#enum = Enum: EnumNode,
    ident = Ident: IdentNode,
    r#loop = Loop: LoopNode,
    objective = Objective: ObjectiveNode,
    import = Import: ImportNode,
    r#return = Return: ReturnNode,
    special = Special: SpecialNode,
    at = At: AtNode,
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
            | Node::Return(_)
            | Node::At(_) => false,

            Node::UnaryOp(_)
            | Node::BinaryOp(_)
            | Node::Literal(_)
            | Node::Call(_)
            | Node::Ident(_)
            | Node::Special(_) => true,
        }
    }
}

use crate::{
    at::AtNode, binop::BinaryOpNode, block::BlockNode, call::CallNode, cond::ConditionalNode, constant::ConstantNode, enums::EnumNode, field::FieldNode, func::FunctionNode, import::ImportNode, literal::LiteralNode, loops::LoopNode, objective::ObjectiveNode, refs::RefNode, ret::ReturnNode, special::SpecialNode, unop::UnaryOpNode, var::VarNode
};

macro_rules! node_data {
    { $name: ident = $($id: ident = $variant: ident: $data: ty,)* } => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpanGroup)]
        pub enum $name<'a> {
            $($variant($data),)*
        }

        $(
            concat_idents::concat_idents!(name = is_, $id {
                impl<'a> $name<'a> {
                    pub fn name(&self) -> bool {
                        match self {
                            Self::$variant(_) => true,
                            _ => false,
                        }
                    }
                }
            });

            concat_idents::concat_idents!(name = as_, $id {
                impl<'a> $name<'a> {
                    pub fn name(&self) -> Option<$data> {
                        match self {
                            Self::$variant(data) => Some(data.clone()),
                            _ => None,
                        }
                    }
                }
            });

            impl<'a> Into<$name<'a>> for $data {
                fn into(self) -> $name<'a> {
                    $name::$variant(self)
                }
            }
        )*

        impl<'a> std::fmt::Display for $name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant(me) => write!(f, "{me}"),)*
                }
            }
        }
    };
}

node_data! {
    Node =
        constant = Constant: ConstantNode<'a>,
        function = Function: FunctionNode<'a>,
        binary_op = BinaryOp: BinaryOpNode<'a>, // TODO: Restrict to assign
        variable = Variable: VarNode<'a>,
        block = Block: BlockNode<'a>,
        call = Call: CallNode<'a>,
        conditional = Conditional: ConditionalNode<'a>,
        r#enum = Enum: EnumNode<'a>,
        r#loop = Loop: LoopNode<'a>,
        objective = Objective: ObjectiveNode<'a>,
        import = Import: ImportNode<'a>,
        r#return = Return: ReturnNode<'a>,
        at = At: AtNode<'a>,
        field = Field: FieldNode<'a>,
        unary_op = UnaryOp: UnaryOpNode<'a>,
        literal = Literal: LiteralNode<'a>,
        special = Special: SpecialNode<'a>,
        r#ref = Ref: RefNode<'a>,
}

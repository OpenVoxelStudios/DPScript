use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::{BuiltInType, TypeRef},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct LiteralNode {
    pub span: SourceSpan,
    pub data: LiteralData,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum LiteralData {
    String(String),
    Int(i64),
    Float(f32),
    Double(f64),
    Bool(bool),
    Array(Vec<Node>),
    // TODO: Time, ident, NBT
}

impl NodeInfo for LiteralNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        true
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        Some(match self.data {
            LiteralData::String(_) => TypeRef::BuiltIn(BuiltInType::String),
            LiteralData::Int(_) => TypeRef::BuiltIn(BuiltInType::Int),
            LiteralData::Float(_) => TypeRef::BuiltIn(BuiltInType::Float),
            LiteralData::Double(_) => TypeRef::BuiltIn(BuiltInType::Double),
            LiteralData::Bool(_) => TypeRef::BuiltIn(BuiltInType::Boolean),

            LiteralData::Array(ref data) => {
                TypeRef::Array(data.iter().map(|it| it.returns(scope)).collect())
            }
        })
    }
}

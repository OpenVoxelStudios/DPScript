use dpscript_ast::prelude::value::{
    binop::{BoolOp, MathOp, Operation},
    unary::UnaryOp,
};

pub const BASE_TYPES_MODULE: &str = "std::types::base";

pub const VOID_TYPE_NAME: &str = "void";
pub const ANY_TYPE_NAME: &str = "Any";
pub const INT_TYPE_NAME: &str = "int";
pub const FLOAT_TYPE_NAME: &str = "float";
pub const DOUBLE_TYPE_NAME: &str = "double";
pub const STR_TYPE_NAME: &str = "str";
pub const BOOL_TYPE_NAME: &str = "bool";
pub const BYTE_TYPE_NAME: &str = "byte";
pub const LONG_TYPE_NAME: &str = "long";
pub const NBT_TYPE_NAME: &str = "NBT";
pub const OBJECTIVE_TYPE_NAME: &str = "Objective";

pub const FUNC_NAME_MATH_ADD: &str = "add";
pub const FUNC_NAME_MATH_SUB: &str = "sub";
pub const FUNC_NAME_MATH_MUL: &str = "mul";
pub const FUNC_NAME_MATH_DIV: &str = "div";
pub const FUNC_NAME_MATH_MOD: &str = "mod";
pub const FUNC_NAME_MATH_NEGATE: &str = "negate";

pub const FUNC_NAME_BOOL_EQ: &str = "eq";
pub const FUNC_NAME_BOOL_NE: &str = "ne";
pub const FUNC_NAME_BOOL_LT: &str = "lt";
pub const FUNC_NAME_BOOL_LE: &str = "le";
pub const FUNC_NAME_BOOL_GT: &str = "gt";
pub const FUNC_NAME_BOOL_GE: &str = "ge";
pub const FUNC_NAME_BOOL_AND: &str = "and";
pub const FUNC_NAME_BOOL_OR: &str = "or";
pub const FUNC_NAME_BOOL_INVERT: &str = "invert";

pub const FUNC_NAME_ARRAY_INDEX: &str = "index";
pub const FUNC_NAME_LOCAL_OFFSET: &str = "local_offset";

// lmao
pub const FUNC_NAME_COMPILER_ERROR: &str = "\0this_function_has_an_invalid_name\0";

pub fn op_to_func(op: Operation) -> &'static str {
    match op {
        Operation::Bool(it) => match it {
            BoolOp::Eq => FUNC_NAME_BOOL_EQ,
            BoolOp::NotEq => FUNC_NAME_BOOL_NE,
            BoolOp::Less => FUNC_NAME_BOOL_LT,
            BoolOp::LessEq => FUNC_NAME_BOOL_LE,
            BoolOp::Greater => FUNC_NAME_BOOL_GT,
            BoolOp::GreaterEq => FUNC_NAME_BOOL_GE,
            BoolOp::And => FUNC_NAME_BOOL_AND,
            BoolOp::Or => FUNC_NAME_BOOL_OR,
        },

        Operation::Math(it) => match it {
            MathOp::Add => FUNC_NAME_MATH_ADD,
            MathOp::Sub => FUNC_NAME_MATH_SUB,
            MathOp::Mul => FUNC_NAME_MATH_MUL,
            MathOp::Div => FUNC_NAME_MATH_DIV,
            MathOp::Mod => FUNC_NAME_MATH_MOD,
        },

        Operation::ArrayIndex => FUNC_NAME_ARRAY_INDEX,
        Operation::None => FUNC_NAME_COMPILER_ERROR,
    }
}

pub fn unary_op_to_func(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Negate => FUNC_NAME_MATH_NEGATE,
        UnaryOp::Invert => FUNC_NAME_BOOL_INVERT,
        UnaryOp::Offset => FUNC_NAME_LOCAL_OFFSET,
        UnaryOp::None => FUNC_NAME_COMPILER_ERROR,
    }
}

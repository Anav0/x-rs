use crate::tokenizer::{Literals, TokenType};

pub trait AstNode {}

pub struct Root;
impl Default for Root {
    fn default() -> Self {
        Self {}
    }
}

impl AstNode for Root {}

pub struct CompoundStmt;

pub struct LiteralDecl {
    pub value: Literals,
}
pub struct VariableDecl {
    pub identifier: String,
}

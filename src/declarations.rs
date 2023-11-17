use crate::tokenizer::{Literals, TokenType};

pub enum NodeType {
    Variable(VariableDecl)
}

pub struct AstNode
{
    pub node_type: NodeType,
}

pub struct Root {
    pub children: Vec<NodeType>
}

pub struct CompoundStmt;

pub struct LiteralDecl {
    pub value: Literals,
}
pub struct VariableDecl {
    pub identifier: String,
    pub literal: LiteralDecl,
}
use crate::tokenizer::{Literals, TokenType};

pub enum NodeType {
    Variable(VariableDecl),
    Stmt(CompoundStmt),
}

pub struct AstNode {
    pub node_type: NodeType,
}

pub struct Root {
    pub children: Vec<NodeType>,
}

pub struct CompoundStmt {
    pub children: Vec<NodeType>,
}
impl Default for CompoundStmt {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

pub struct LiteralDecl {
    pub value: Literals,
}
pub struct VariableDecl {
    pub identifier: String,
    pub literal: LiteralDecl,
}

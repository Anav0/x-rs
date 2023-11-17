
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

// STATEMENTS

pub struct CompoundStmt {
    pub stack_offset: u16,
    pub children: Vec<NodeType>,
}

impl CompoundStmt {
    pub fn new(stack_offset: u16) -> Self {
        Self { stack_offset, children: vec![] }
    }
}

// LITERALS

pub enum Literals {
    NUMBER(String),
    STR(String),
}

pub struct LiteralDecl {
    pub value: Literals,
}

pub struct VariableDecl {
    pub stack_offset: u16,
    pub identifier: String,
    pub literal: LiteralDecl,
}

pub struct FunctionDecl {
    pub stack_offset: u16,
    pub identifier: String,
    pub literal: LiteralDecl,
}

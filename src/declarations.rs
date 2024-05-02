
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Variable(VariableDecl),
    Stmt(CompoundStmt),
}

#[derive(Debug, PartialEq)]
pub struct CompoundStmt {
    pub stack_offset: u16,
    pub children: Vec<usize>,
}

impl CompoundStmt {
    pub fn new(stack_offset: u16) -> Self {
        Self { stack_offset, children: vec![] }
    }
}

// LITERALS

#[derive(Debug, PartialEq)]
pub enum Literals {
    NUMBER(String),
    STR(String),
}

#[derive(Debug, PartialEq)]
pub struct LiteralDecl {
    pub value: Literals,
}

#[derive(Debug, PartialEq)]
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

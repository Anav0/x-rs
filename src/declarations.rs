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
        Self {
            stack_offset,
            children: vec![],
        }
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

pub struct IncDecl {
    // Which literal is incremented
    pub literal_index: u32,
    pub literal_scope_index: u32,
}

pub struct LoopDecl {
    pub jmp_label_declaration: String, //for(u8 i = 12; i < 100; i++)
    //---------^------------------
    pub jmp_label_conditional: String, //for(u8 i = 12; i < 100; i++)
    //-----------------^----------
    pub jmp_label_body: String,
}

use std::fs;

use crate::{
    declarations::{NodeType, VariableDecl},
    parser::AST,
    tokenizer::Literals,
};

pub struct Assembler {
    output: String,
    filename: String,
}

impl Assembler {
    pub fn new(filename: &str) -> Self {
        let mut assem = Self {
            output: String::with_capacity(4096),
            filename: filename.to_string(),
        };

        assem.output += "PUSH rbp\n";
        assem.output += "MOV  rbp rsp\n";

        assem
    }

    pub fn foo(&mut self, nodes: &Vec<NodeType>) {
        for node_type in nodes {
            match node_type {
                NodeType::Variable(variableDecl) => {}
                NodeType::Stmt(stmt) => return self.foo(&stmt.children),
            }
        }
    }

    pub fn from_ast(&mut self, ast: &AST) {
        self.foo(&ast.root.children)
    }

    pub fn var(&mut self, variableDecl: VariableDecl) {
        let var_type = match variableDecl.literal.value {
            Literals::NUMBER(_) => "QWORD",
            Literals::STR(_) => "QWORD",
        };

        let var_size: u8 = match variableDecl.literal.value {
            Literals::NUMBER(_) => 16,
            Literals::STR(_) => 32,
        };

        let var_value = match variableDecl.literal.value {
            Literals::NUMBER(value) => value.to_string(),
            Literals::STR(value) => value,
        };

        self.output += &format!("{var_type} PTR [rbp={var_size}], {var_value}");
    }

    pub fn build(&mut self) {
        fs::write("./out.asm", &self.output).expect("Failed to save asm file!");
        self.output.clear();
    }
}

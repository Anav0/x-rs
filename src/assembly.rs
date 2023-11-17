use std::fs;

use crate::{
    declarations::{NodeType, VariableDecl, Literals},
    parser::{AST, get_var_size},
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

    pub fn assemble_based_on_nodes(&mut self, nodes: &Vec<NodeType>) {
        for node_type in nodes {
            match node_type {
                NodeType::Variable(variable_decl) => self.var(variable_decl),
                NodeType::Stmt(stmt) => self.assemble_based_on_nodes(&stmt.children),
            }
        }
    }

    pub fn from_ast(&mut self, ast: &AST) {
        self.assemble_based_on_nodes(&ast.root.children)
    }

    pub fn var(&mut self, variable_decl: &VariableDecl) {
        let var_type = match variable_decl.literal.value {
            Literals::NUMBER(_) => "QWORD",
            Literals::STR(_) => panic!("String is not yet supported as a variable declaration type"),
        };

        let var_size: u16 = get_var_size(variable_decl);

        let offset = variable_decl.stack_offset + var_size;

        let var_value = match &variable_decl.literal.value {
            Literals::NUMBER(value) => value,
            Literals::STR(value) => value,
        };

        let v = &variable_decl.identifier;
        //TODO: move memory offset with each subsequent variable declaration
        self.output += &format!("{var_type} PTR [rbp-{offset}], {var_value} ; {v}\n");
    }

    pub fn build(&mut self) {
        fs::write("./out.asm", &self.output).expect("Failed to save asm file!");
        self.output.clear();
    }
}

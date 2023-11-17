use crate::{
    declarations::{Literals, CompoundStmt, LiteralDecl, NodeType, Root, VariableDecl},
};

use std::{
    iter::Peekable,
};

//TODO: move
pub fn get_var_size(decl: &VariableDecl) -> u16 {
 match decl.literal.value {
            Literals::NUMBER(_) => 16, //NOTE: for now all numbers have the same size
            Literals::STR(_) => 32,
        }
    }

use crate::{
    symbols::CodeScope,
    tokenizer::{Token, TokenType, Tokenizer},
};

pub struct AST {
    pub root: Root,
    pub nodes: Vec<NodeType>,
}

pub struct Parser {
    scopes: Vec<CodeScope>,
}

impl Parser {
    pub fn new(scope: CodeScope) -> Self {
        Self {
            scopes: vec![scope],
        }
    }

    pub fn parse(&mut self, tokenizer: Tokenizer) -> AST {
        let mut root = Root { children: vec![] };
        let mut ast = AST {
            root,
            nodes: vec![],
        };

        let chars = &tokenizer.chars.clone(); //TODO: get rid of this clone!
        let mut iter = tokenizer.peekable();
        let mut current_scope_index: Option<usize> = Some(0);

        let mut new_scope = CodeScope::new(None);
        while let Some(current_token) = iter.next() {
            if current_token.token_type == TokenType::LeftBrace {
                new_scope = CodeScope::new(current_scope_index);
                let scope_index = self.scopes.len();
                self.scopes.push(new_scope.clone());
                self.scopes[current_scope_index.unwrap()].add_nested_scope(scope_index);

                current_scope_index = Some(scope_index);
                continue;
            }

            if current_token.token_type == TokenType::RightBrace {
                if current_scope_index.is_none() {
                    panic!("Lone right brace!")
                }

                let current_scope = &mut self.scopes[current_scope_index.unwrap()];

                if current_scope.was_closed {
                    panic!("Scope was already closed!")
                }

                current_scope.was_closed = true;
                current_scope_index = current_scope.parent_scope;
                continue;
            }

            let stmt = self
                .match_stmt(&mut new_scope, current_token, &mut iter, chars, &mut ast)
                .expect("Failed to match statment");

            ast.root.children.push(NodeType::Stmt(stmt));
        }

        ast
    }

    fn match_stmt(
        &mut self,
        scope: &mut CodeScope,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        chars: &Vec<char>,
        ast: &mut AST,
    ) -> Option<CompoundStmt> {
        let mut stmt = CompoundStmt::new(0);

        return match current_token.token_type {
            TokenType::LET => {
                dbg!("Matched LET. Starting to parse assigment");
                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No tokens after 'let' keyword")
                }

                let identifier = self
                    .match_ident(&next_token.unwrap())
                    .expect("Failed to match ident!");

                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No token after identifier!");
                }

                if next_token.unwrap().token_type != TokenType::EQUAL {
                    panic!("No '=' found in assigment statment!");
                }

                let next_token = tokenizer.next()?;

                let literal_value = self
                    .match_literal(next_token, tokenizer, chars, ast)
                    .expect("Failed to match literal");

                let literal = LiteralDecl {
                    value: literal_value,
                };

                let decl = VariableDecl {
                    stack_offset: scope.stack_pointer,
                    identifier,
                    literal,
                };

                scope.stack_pointer += get_var_size(&decl);

                stmt.children.push(NodeType::Variable(decl));

                return Some(stmt);
            }
            _ => None,
        };
    }

    

    fn match_literal(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        chars: &Vec<char>,
        ast: &mut AST,
    ) -> Option<Literals> {
        if self.match_digit(&current_token) {
            let value: String = chars[current_token.start..current_token.end]
                .into_iter()
                .collect();
            return Some(Literals::NUMBER(value));
        }

        None
    }

    fn match_oper(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        ast: &mut AST,
    ) -> bool {
        false
    }

    fn match_digit(&mut self, current_token: &Token) -> bool {
        return match current_token.token_type {
            TokenType::NUMBER(_) => true,
            _ => false,
        };
    }

    fn match_ident(&mut self, current_token: &Token) -> Option<String> {
        return match &current_token.token_type {
            TokenType::IDENT(value) => Some(value.clone()),
            _ => None,
        };
    }
}

use crate::{
    assembly::Assembler,
    declarations::{AstNode, CompoundStmt, LiteralDecl, NodeType, Root, VariableDecl},
    tokenizer::Literals,
};

use std::{
    fmt,
    iter::Peekable,
    thread::{current, scope},
};

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

        while let Some(current_token) = iter.next() {
            if current_token.token_type == TokenType::LeftBrace {
                let new_scope = CodeScope::new(current_scope_index);
                let scope_index = self.scopes.len();
                self.scopes.push(new_scope);
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

            let stmt = self.match_stmt(current_token, &mut iter, chars, &mut ast).expect("Failed to match statment");

            ast.root.children.push(NodeType::Stmt(stmt));
        }

        ast
    }

    fn match_stmt(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        chars: &Vec<char>,
        ast: &mut AST,
    ) -> Option<CompoundStmt> {
        let mut stmt = CompoundStmt::default();

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

                let literalValue = self
                    .match_literal(current_token, tokenizer, chars, ast)
                    .expect("Failed to match literal");

                let next_token = tokenizer.next();
                if !next_token.is_none() {
                    let literal = LiteralDecl {
                        value: literalValue,
                    };

                    let decl = VariableDecl {
                        identifier,
                        literal,
                    };

                    stmt.children.push(NodeType::Variable(decl));

                    return Some(stmt);
                    //return next_token.unwrap().token_type == TokenType::COMMA;
                }

                None
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

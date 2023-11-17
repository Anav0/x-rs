use crate::declarations::{AstNode, NodeType, Root};

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

            if !self.match_stmt(current_token, &mut iter, &mut ast) {
                panic!("Failed to match statment");
            }
        }
        dbg!(&self.scopes);
        ast
    }

    fn match_stmt(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        ast: &mut AST,
    ) -> bool {
        return match current_token.token_type {
            TokenType::LET => {
                dbg!("Matched LET. Starting to parse assigment");
                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No tokens after 'let' keyword")
                }

                if !self.match_ident(&next_token.unwrap()) {
                    panic!("Failed to match expr");
                }

                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No token after identifier!");
                }

                if next_token.unwrap().token_type != TokenType::EQUAL {
                    panic!("No '=' found in assigment statment!");
                }

                if !self.match_expr(tokenizer, ast) {
                    panic!("Failed to match expr in assigment");
                }

                let next_token = tokenizer.next();
                if !next_token.is_none() {
                    return next_token.unwrap().token_type == TokenType::COMMA;
                }

                false
            }
            _ => false,
        };
    }

    fn match_optexpr(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        ast: &mut AST,
    ) -> bool {
        false
    }

    fn match_expr(&mut self, tokenizer: &mut Peekable<Tokenizer>, ast: &mut AST) -> bool {
        let mut next_token = tokenizer.next();

        if next_token.is_none() {
            return false;
        }

        let next_token = next_token.unwrap();

        false
    }

    fn match_term(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        ast: &mut AST,
    ) -> bool {
        return self.match_digit(&current_token) || self.match_ident(&current_token);
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
    fn match_ident(&mut self, current_token: &Token) -> bool {
        return match current_token.token_type {
            TokenType::IDENT(_) => true,
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        declarations::Root,
        parser::Node,
        symbols::CodeScope,
        tokenizer::{Token, TokenType, Tokenizer},
    };

    #[test]
    fn parses_variables() {
        let tokenizer = Tokenizer::new("let x = 2;");
        let mut scope = CodeScope::new(None);
        let mut parser = Parser::new(scope);

        let ast: super::AST<'_, Root> = parser.parse(tokenizer);
    }
}

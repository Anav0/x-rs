use std::{
    fmt,
    iter::Peekable,
    thread::{current, scope},
};

use crate::{
    symbols::CodeScope,
    tokenizer::{Token, TokenType, Tokenizer},
};

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,

    value: Option<Token>,
}

pub struct Parser<'a> {
    scopes: Vec<CodeScope<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(scope: CodeScope<'a>) -> Self {
        Self {
            scopes: vec![scope],
        }
    }

    pub fn parse(&mut self, tokenizer: Tokenizer) -> AST {
        let mut ast = AST { nodes: vec![] };
        let mut iter = tokenizer.peekable();
        let mut seen_left_brace = false;
        let mut current_scope_index = 0;

        while let Some(current_token) = iter.next() {
            if current_token.token_type == TokenType::LeftBrace {
                if seen_left_brace {
                    panic!("Two consecutive left braces!")
                }

                let scope = CodeScope::new(Some(current_scope_index));
                let scope_index = self.scopes.len();
                self.scopes.push(scope);
                self.scopes[current_scope_index].add_nested_scope(scope_index);

                seen_left_brace = true;
            }

            if current_token.token_type == TokenType::RightBrace {
                if !seen_left_brace {
                    panic!("Right brace without the right one!")
                }

                let parent = self.scopes[current_scope_index].parent_scope;

                if !parent.is_none() {
                    current_scope_index = parent.unwrap();
                }

                seen_left_brace = false;
            }

            if !self.match_stmt(current_token, &mut iter, &mut ast) {
                panic!("Failed to match statment");
            }
        }
        ast
    }

    fn match_stmt(
        &mut self,
        current_token: Token,
        tokenizer: &mut Peekable<Tokenizer>,
        ast: &mut AST,
    ) -> bool {
        return match current_token.token_type {
            //stmt -> ident = expr ;
            //        | if ( expr ) stmt
            //        | if ( expr ) stmt else stmt
            // ident = expr ;
            TokenType::LET => {
                dbg!("Matched LET");
                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No tokens after 'let' keyword")
                }

                if !self.match_ident(&next_token.unwrap()) {
                    panic!("Failed to match expr");
                }

                let next_token = tokenizer.next();

                if next_token.is_none() {
                    panic!("No token after assigment!");
                }

                if next_token.unwrap().token_type != TokenType::EQUAL {
                    panic!("No equal found in assigment statment!");
                }

                if !self.match_expr(tokenizer, ast) {
                    panic!("Failed to match expr in assigment");
                }

                //TODO: check for ;
                true
            }
            TokenType::IF => false,
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
        // expr -> expr oper term | term
        let mut next_token = tokenizer.next();

        if next_token.is_none() {
            return false;
        }

        let next_token_un = next_token.unwrap();

        let potential_term = next_token_un.clone();

        //expr oper term
        if self.match_expr(tokenizer, ast) {
            dbg!("Matched expr");
            next_token = tokenizer.next();

            if next_token.is_none() {
                return false;
            }

            if self.match_oper(next_token.unwrap(), tokenizer, ast) {
                return false;
            }

            next_token = tokenizer.next();

            if next_token.is_none() {
                return false;
            }

            if self.match_term(next_token.unwrap(), tokenizer, ast) {
                return false;
            }
        }

        // term
        if self.match_term(potential_term, tokenizer, ast) {
            dbg!("Matched term");
            return true;
        }

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
        parser::Node,
        symbols::CodeScope,
        tokenizer::{Token, TokenType, Tokenizer},
    };

    #[test]
    fn parses_simple_expressions() {
        let tokenizer = Tokenizer::new("let a = 2 + 2;");
        let mut scope = CodeScope::new(None);
        let mut parser = Parser::new(scope);

        let ast = parser.parse(tokenizer);

        let nodes = vec![
            Node {
                parent: None,
                left: None,
                right: None,
                value: Some(Token {
                    token_type: TokenType::LET,
                    start: 0,
                    end: 2,
                }),
            },
            Node {
                parent: None,
                left: Some(1),
                right: Some(2),
                value: Some(Token {
                    token_type: TokenType::IDENT(String::from("a")),
                    start: 4,
                    end: 4,
                }),
            },
            Node {
                parent: None,
                left: Some(1),
                right: Some(2),
                value: Some(Token {
                    token_type: TokenType::EQUAL,
                    start: 6,
                    end: 6,
                }),
            },
            Node {
                parent: Some(0),
                left: Some(2),
                right: Some(3),
                value: Some(Token {
                    token_type: TokenType::PLUS,
                    start: 10,
                    end: 10,
                }),
            },
            Node {
                parent: Some(2),
                left: None,
                right: None,
                value: Some(Token {
                    token_type: TokenType::NUMBER(2),
                    start: 8,
                    end: 8,
                }),
            },
            Node {
                parent: Some(2),
                left: None,
                right: None,
                value: Some(Token {
                    token_type: TokenType::NUMBER(2),
                    start: 12,
                    end: 12,
                }),
            },
        ];

        dbg!(ast);
        todo!();
        assert_eq!(ast.nodes.len(), nodes.len());

        let mut i = 0;
        for expected_node in nodes {
            assert_eq!(ast.nodes[i], expected_node);
            i += 1;
        }
    }
}

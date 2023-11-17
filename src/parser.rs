use std::fmt;

use crate::tokenizer::{Token, TokenType, Tokenizer};

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,

    value: Option<Token>,
}

impl Node {
    pub fn root() -> Self {
        Self {
            parent: None,
            left: None,
            right: None,
            value: None,
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, tokenizer: Tokenizer) -> AST {
        let mut ast = AST {
            nodes: vec![Node::root()],
        };
        let mut iter = tokenizer.peekable();
        let mut prev_token: Option<Token> = None;
        while let Some(current_token) = iter.next() {
            let next_token = iter.peek();
            match current_token.token_type {
                TokenType::EQUAL
                | TokenType::PLUS
                | TokenType::MINUS
                | TokenType::MULT
                | TokenType::DIV => {
                    dbg!(current_token);
                    if !self.are_operands_valid(prev_token, next_token) {
                        panic!("Invalid operands");
                    }
                    let next_token = iter.next();
                    let mut new_operation_node = Node {
                        parent: None,
                        left: None,
                        right: None,
                        value: Some(current_token),
                    };
                    let left_operand_node = Node {
                        parent: Some(ast.nodes.len()),
                        left: None,
                        right: None,
                        value: prev_token,
                    };
                    let right_operand_node = Node {
                        parent: Some(ast.nodes.len()),
                        left: None,
                        right: None,
                        value: next_token,
                    };

                    ast.nodes.push(left_operand_node);
                    ast.nodes.push(right_operand_node);
                    new_operation_node.left = Some(ast.nodes.len() - 2);
                    new_operation_node.right = Some(ast.nodes.len() - 1);
                    ast.nodes.push(new_operation_node);

                    prev_token = next_token;
                }
                _ => {
                    prev_token = Some(current_token);
                }
            }
        }
        ast
    }

    fn are_operands_valid(
        &self,
        prev_token_type: Option<Token>,
        next_token: Option<&Token>,
    ) -> bool {
        if next_token.is_none() {
            panic!("Operator requires right operand");
        }
        if prev_token_type.is_none() {
            panic!("Operator requires left operand");
        }

        let mut is_valid = match prev_token_type.unwrap().token_type {
            TokenType::IDENT | TokenType::NUMBER | TokenType::STRING => true,
            _ => false,
        };

        if !is_valid {
            dbg!(prev_token_type);
            dbg!(next_token);
            panic!("Left operand is not a valid type");
        };

        is_valid = match next_token.unwrap().token_type {
            TokenType::IDENT | TokenType::NUMBER | TokenType::STRING => true,
            _ => false,
        };

        if !is_valid {
            panic!("Right operand is not a valid type");
        };

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{self, Tokenizer};

    use super::Parser;

    #[test]
    fn parse_math() {
        let tokenizer = Tokenizer::new("let a = 2 + 2;");
        let mut parser = Parser::new();

        let ast = parser.parse(tokenizer);

        assert_eq!(ast.nodes.len(), 6);
    }
}

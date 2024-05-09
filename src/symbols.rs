use std::collections::HashMap;

use crate::tokenizer::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct CodeScope {
    pub stack_pointer: u16,
    pub symbols: HashMap<String, Token>,
    pub was_closed: bool,
    pub parent_scope: Option<usize>,

    nested_scopes: Vec<usize>,
}

impl CodeScope {
    pub fn global() -> Self {
        Self {
            stack_pointer: 0,
            symbols: HashMap::new(),
            nested_scopes: vec![],
            parent_scope: None,
            was_closed: false,
        }
    }
    pub fn new(parent_scope: Option<usize>) -> Self {
        Self {
            stack_pointer: 0,
            symbols: HashMap::new(),
            nested_scopes: vec![],
            parent_scope,
            was_closed: false,
        }
    }

    pub fn add_nested_scope(&mut self, nested_scope_index: usize) {
        self.nested_scopes.push(nested_scope_index);
    }

    pub fn add_symbol(&mut self, token: Token) {
        match token.token_type {
            TokenType::IDENT(ref symbol) => {
                if self.symbols.contains_key(symbol) {
                    panic!("Symbol {symbol} already exists in this scope");
                }
                self.symbols.insert(symbol.clone(), token);
            }
            _ => {
                panic!("Cannot add non identifiers as symbols")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, TokenType};

    use super::CodeScope;

    #[should_panic]
    #[test]
    fn add_symbol_do_not_allow_duplicates() {
        let mut scope = CodeScope::new(None);

        scope.add_symbol(Token {
            token_type: TokenType::IDENT(String::from("a")),
            start: 0,
            len: 1,
        });

        scope.add_symbol(Token {
            token_type: TokenType::IDENT(String::from("a")),
            start: 2,
            len: 1,
        });
    }

    #[test]
    fn add_different_symbol() {
        let mut scope = CodeScope::new(None);

        scope.add_symbol(Token {
            token_type: TokenType::IDENT(String::from("a")),
            start: 0,
            len: 1,
        });

        scope.add_symbol(Token {
            token_type: TokenType::IDENT(String::from("b")),
            start: 1,
            len: 1,
        });

        assert_eq!(true, scope.symbols.contains_key("a"));
        assert_eq!(true, scope.symbols.contains_key("b"));
        assert_eq!(2, scope.symbols.len());
    }
}

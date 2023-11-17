use std::{
    collections::{HashMap, HashSet, VecDeque},
    marker::PhantomData,
};

use crate::tokenizer::{Token, TokenType};

type Stack<T> = VecDeque<T>;

#[derive(Debug)]
pub struct CodeScope {
    pub symbols: HashMap<String, Token>,

    pub was_closed: bool,

    pub parent_scope: Option<usize>,
    nested_scopes: Vec<usize>,
}

impl CodeScope {
    pub fn global() -> Self {
        Self {
            symbols: HashMap::new(),
            nested_scopes: vec![],
            parent_scope: None,
            was_closed: false,
        }
    }
    pub fn new(parent_scope: Option<usize>) -> Self {
        Self {
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
            end: 0,
        });

        scope.add_symbol(Token {
            token_type: TokenType::IDENT(String::from("a")),
            start: 0,
            end: 0,
        });
    }
}

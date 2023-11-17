use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

const MAX_IDENT_LENGTH: u32 = 1024;
const COMMENT_CHAR: char = '#';

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    ROOT,

    // General
    IDENT,
    KEYWORD,
    LITERALS,
    SEPARATOR,
    COMMENT,

    // Types
    STRING,
    NUMBER,

    // Operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    EQUAL,

    ARROW,
    FN,
    BRACKET_LEFT,
    BRACKET_RIGHT,
}

pub union NumberTokenValue {
    float_32: f32,
    int_32: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub ident_length: u32,
}

pub struct Tokenizer {
    pub index: usize,

    chars: Vec<char>,
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut section: Vec<char> = vec![];

        let mut is_next_token = false;
        let mut skip_comment = false;

        for i in self.index..self.chars.len() {
            let char = self.chars[i];
            self.index += 1;

            is_next_token = false;

            if skip_comment && char != '\n' {
                continue;
            } else {
                skip_comment = false;
            }

            match char {
                '#' => skip_comment = true,
                '=' | ';' | ' ' | '\n' | '\r' | '\t' | '{' | '}' => is_next_token = true,
                _ => {}
            }

            if skip_comment {
                continue;
            }

            if is_next_token && section.len() > 0 {
                let mut token_type = match section[0] {
                    '=' => TokenType::EQUAL,
                    '-' => TokenType::MINUS,
                    '+' => TokenType::PLUS,
                    '*' => TokenType::MULT,
                    '/' => TokenType::DIV,
                    _ => TokenType::IDENT,
                };

                let ident = section.iter().collect::<String>();
                let mut is_numeric = true;
                for c in ident.chars() {
                    if !c.is_digit(10) {
                        is_numeric = false;
                        break;
                    }
                }
                if is_numeric {
                    token_type = TokenType::NUMBER;
                }

                if ident == "let" {
                    token_type = TokenType::KEYWORD;
                }
                section.clear();

                return Some(Token {
                    token_type,
                    ident_length: 0,
                });
            } else if !char.is_whitespace() {
                section.push(char);
            }
        }
        None
    }
}

impl Tokenizer {
    pub fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            index: 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenType::{DIV, EQUAL, IDENT, KEYWORD, MINUS, MULT, NUMBER, PLUS};
    use crate::tokenizer::{Token, Tokenizer};

    #[test]
    fn produces_correct_tokens() {
        let tokenizer = Tokenizer::new("let a = 2 + 2 - 2 * 2 / 2;");

        let tokens: Vec<Token> = tokenizer.collect();

        let expected_types = vec![
            KEYWORD, IDENT, EQUAL, NUMBER, PLUS, NUMBER, MINUS, NUMBER, MULT, NUMBER, DIV, NUMBER,
        ];

        assert_eq!(tokens.len(), expected_types.len());

        let mut i = 0;
        for token in tokens {
            let expected_token = expected_types[i];
            assert_eq!(token.token_type, expected_token);
            i += 1;
        }
    }

    #[test]
    fn ignores_comments() {
        let tokenizer = Tokenizer::new("#some comment on first line\nlet a = 2 + 2;");

        let tokens: Vec<Token> = tokenizer.collect();

        let expected_types = vec![KEYWORD, IDENT, EQUAL, NUMBER, PLUS, NUMBER];

        assert_eq!(tokens.len(), expected_types.len());
    }

    #[test]
    fn ignores_comments_even_with_valid_expressions() {
        let tokenizer = Tokenizer::new("#some comment on first line let a = 2 + 2;");

        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn respects_valid_delimiters() {
        let valid_delimeters = vec![" ", "    ", "\n", ";", "\t"];

        for delimiter in valid_delimeters {
            let tokenizer = Tokenizer::new(&format!("let a = 2{delimiter}let b = 10;"));

            let tokens: Vec<Token> = tokenizer.collect();

            let expected_types = vec![KEYWORD, IDENT, EQUAL, NUMBER, KEYWORD, IDENT, EQUAL, NUMBER];

            assert_eq!(tokens.len(), expected_types.len());

            let mut i = 0;
            for token in tokens {
                let expected_token = expected_types[i];
                assert_eq!(token.token_type, expected_token);
                i += 1;
            }
        }
    }
    #[test]
    fn not_text_no_tokens() {
        let no_token_texts = vec!["", " ", "    ", "\n", ";", "\t", "#", "#ABDE", "#let x = 2"];

        for text in no_token_texts {
            let tokenizer = Tokenizer::new(text);

            let tokens: Vec<Token> = tokenizer.collect();

            assert_eq!(tokens.len(), 0);
        }
    }
}

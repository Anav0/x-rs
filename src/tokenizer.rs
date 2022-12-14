const COMMENT_CHAR: char = '#';

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // General
    IDENT(String),
    COMMENT,

    // Types
    STRING(String),
    NUMBER(u32),

    //Separators
    LeftBrace,
    RightBrace,

    // Operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    EQUAL,

    // Keywords
    IF,
    LET,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer {
    pub index: usize,
    chars: Vec<char>,
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut token_buffer: Vec<char> = Vec::with_capacity(256);

        let mut current_token_ends = false;
        let mut skip_comment = false;

        for i in self.index..self.chars.len() {
            let char = self.chars[i];
            self.index += 1;

            current_token_ends = false;

            if skip_comment && char != '\n' {
                continue;
            } else {
                skip_comment = false;
            }

            match char {
                COMMENT_CHAR => skip_comment = true,
                '=' | ';' | ' ' | '\n' | '\r' | '\t' | '{' | '}' => current_token_ends = true,
                _ => {}
            }

            if skip_comment {
                continue;
            }

            if current_token_ends && token_buffer.len() > 0 {
                let ident = token_buffer.iter().collect::<String>();
                let mut token_type = match token_buffer[0] {
                    '=' => TokenType::EQUAL,
                    '-' => TokenType::MINUS,
                    '+' => TokenType::PLUS,
                    '*' => TokenType::MULT,
                    '/' => TokenType::DIV,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    _ => TokenType::IDENT(ident.clone()),
                };

                let mut is_numeric = true;
                for c in &token_buffer {
                    if !c.is_digit(10) {
                        is_numeric = false;
                        break;
                    }
                }

                if is_numeric {
                    let number =
                        u32::from_str_radix(&ident, 10).expect("Failed to parse number as u32");
                    token_type = TokenType::NUMBER(number);
                }

                if ident == "let" {
                    token_type = TokenType::LET;
                }

                if ident == "if" {
                    token_type = TokenType::IF;
                }

                token_buffer.clear();

                return Some(Token {
                    token_type,
                    start: i - token_buffer.len() - 1,
                    end: i,
                });
            } else if !char.is_whitespace() {
                token_buffer.push(char);
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
    pub fn get_chars(&self) -> &Vec<char> {
        &self.chars
    }
}
#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenType::{DIV, EQUAL, IDENT, LET, MINUS, MULT, NUMBER, PLUS};
    use crate::tokenizer::{Token, Tokenizer};

    #[test]
    fn produces_correct_tokens() {
        let tokenizer = Tokenizer::new("let a = 2 + 2 - 2 * 2 / 2;");

        let tokens: Vec<Token> = tokenizer.collect();

        let expected_types = vec![
            LET,
            IDENT(String::from("a")),
            EQUAL,
            NUMBER(2),
            PLUS,
            NUMBER(2),
            MINUS,
            NUMBER(2),
            MULT,
            NUMBER(2),
            DIV,
            NUMBER(2),
        ];

        assert_eq!(tokens.len(), expected_types.len());

        let mut i = 0;
        for token in tokens {
            let expected_token = &expected_types[i];
            assert_eq!(token.token_type, *expected_token);
            i += 1;
        }
    }

    #[test]
    fn ignores_comments() {
        let tokenizer = Tokenizer::new("#some comment on first line\nlet a = 2 + 2;");

        let tokens: Vec<Token> = tokenizer.collect();

        let expected_types = vec![
            LET,
            IDENT(String::from("a")),
            EQUAL,
            NUMBER(2),
            PLUS,
            NUMBER(2),
        ];

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

            let expected_types = vec![
                LET,
                IDENT(String::from("a")),
                EQUAL,
                NUMBER(2),
                LET,
                IDENT(String::from("b")),
                EQUAL,
                NUMBER(10),
            ];

            assert_eq!(tokens.len(), expected_types.len());

            let mut i = 0;
            for token in tokens {
                let expected_token = &expected_types[i];
                assert_eq!(token.token_type, *expected_token);
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

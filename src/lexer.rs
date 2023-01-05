use crate::token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            current_char: '\x00',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\x00';
        } else {
            self.current_char = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\x00';
        } else {
            return self.input.as_bytes()[self.read_position] as char;
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let token = match self.current_char {
            '=' => {
                if self.peak_char() == '=' {
                    let prev_char = self.current_char;
                    self.read_char();
                    token::Token {
                        token_type: token::TokenType::EQ,
                        literal: format!("{}{}", prev_char, self.current_char),
                    }
                } else {
                    token::Token {
                        token_type: token::TokenType::ASSIGN,
                        literal: String::from(self.current_char),
                    }
                }
            }
            ';' => token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: String::from(self.current_char),
            },
            '(' => token::Token {
                token_type: token::TokenType::LPAREN,
                literal: String::from(self.current_char),
            },
            ')' => token::Token {
                token_type: token::TokenType::RPAREN,
                literal: String::from(self.current_char),
            },
            ',' => token::Token {
                token_type: token::TokenType::COMMA,
                literal: String::from(self.current_char),
            },
            '+' => token::Token {
                token_type: token::TokenType::PLUS,
                literal: String::from(self.current_char),
            },
            '-' => token::Token {
                token_type: token::TokenType::MINUS,
                literal: String::from(self.current_char),
            },
            '!' => {
                if self.peak_char() == '=' {
                    let prev_char = self.current_char;
                    self.read_char();
                    token::Token {
                        token_type: token::TokenType::NOTEQ,
                        literal: format!("{}{}", prev_char, self.current_char),
                    }
                } else {
                    token::Token {
                        token_type: token::TokenType::BANG,
                        literal: String::from(self.current_char),
                    }
                }
            },
            '*' => token::Token {
                token_type: token::TokenType::ASTERISK,
                literal: String::from(self.current_char),
            },
            '<' => token::Token {
                token_type: token::TokenType::LT,
                literal: String::from(self.current_char),
            },
            '>' => token::Token {
                token_type: token::TokenType::GT,
                literal: String::from(self.current_char),
            },
            '/' => token::Token {
                token_type: token::TokenType::SLASH,
                literal: String::from(self.current_char),
            },
            '{' => token::Token {
                token_type: token::TokenType::LBRACE,
                literal: String::from(self.current_char),
            },
            '}' => token::Token {
                token_type: token::TokenType::RBRACE,
                literal: String::from(self.current_char),
            },
            '\0' => token::Token {
                token_type: token::TokenType::EOF,
                literal: String::from("\x00"),
            },
            character => {
                let literal = self.read_identifier();
                let token_type = self.lookup_token_type(&literal);

                return if character.is_alphabetic() {
                    token::Token {
                        token_type,
                        literal,
                    }
                } else if character.is_digit(10) {
                    token::Token {
                        token_type: token::TokenType::INT,
                        literal: self.read_number(),
                    }
                } else {
                    token::Token {
                        token_type: token::TokenType::ILLEGAL,
                        literal: String::from(character),
                    }
                };
            }
        };

        self.read_char();
        return token;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.current_char.is_alphabetic() || self.current_char == '_' {
            self.read_char();
        }

        let literal = &self.input[position..self.position];

        return String::from(literal);
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.current_char.is_digit(10) {
            self.read_char();
        }

        let literal = &self.input[position..self.position];

        return String::from(literal);
    }

    fn lookup_token_type(&self, ident: &str) -> token::TokenType {
        match ident {
            "fn" => token::TokenType::FUNCTION,
            "let" => token::TokenType::LET,
            "true" => token::TokenType::TRUE,
            "false" => token::TokenType::FALSE,
            "if" => token::TokenType::IF,
            "else" => token::TokenType::ELSE,
            "return" => token::TokenType::RETURN,
            _ => token::TokenType::IDENT,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

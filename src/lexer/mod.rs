#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Literals
    Integer(i32),
    Float(f64),
    String(String),
    Bool(bool),

    // Collections
    Channel,
    Send,
    Recv,
    Push,
    Pop,
    Vec,
    HashMap,

    // Keywords
    Let,
    Func,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Mod,
    Pub,
    Use,
    Struct,
    Impl,
    Async,
    Await,

    // Memory Management
    Tilde, // ~ (unique ownership)
    At,    // @ (shared ownership)

    // Attributes
    WeakAttr,  // #weak
    SyncAttr,  // #sync
    OwnAttr,   // #own
    ActorAttr, // #actor

    // Types
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeBool,
    TypeString,
    TypeDyn,
    TypeVec,
    TypeHashMap,

    // Operators and Delimiters
    Plus,
    PlusPlus,
    PlusEq,
    Minus,
    MinusMinus,
    MinusEq,
    Multiply,
    Divide,
    Assign,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    Modulus,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    DoubleColon,
    Semicolon,
    Arrow,

    // Special
    Identifier(String),
    Eof,
    Invalid(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().cloned();

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(number.parse().unwrap())
        } else {
            Token::Integer(number.parse().unwrap())
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            // Keywords
            "let" => Token::Let,
            "func" => Token::Func,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "return" => Token::Return,
            "mod" => Token::Mod,
            "pub" => Token::Pub,
            "use" => Token::Use,
            "struct" => Token::Struct,
            "impl" => Token::Impl,
            "async" => Token::Async,
            "await" => Token::Await,

            // Types
            "i8" => Token::TypeI8,
            "i16" => Token::TypeI16,
            "i32" => Token::TypeI32,
            "i64" => Token::TypeI64,
            "u8" => Token::TypeU8,
            "u16" => Token::TypeU16,
            "u32" => Token::TypeU32,
            "u64" => Token::TypeU64,
            "f32" => Token::TypeF32,
            "f64" => Token::TypeF64,
            "bool" => Token::TypeBool,
            "string" => Token::TypeString,
            "dyn" => Token::TypeDyn,

            // Concurrency
            "channel" => Token::Channel,
            "send" => Token::Send,
            "recv" => Token::Recv,
            // "push" => Token::Push,
            // "pop" => Token::Pop,

            // Collections
            "Vec" => Token::Vec,
            "HashMap" => Token::HashMap,

            // Default case
            _ => Token::Identifier(identifier),
        }
    }

    fn read_attribute(&mut self) -> Token {
        self.advance();
        let mut attr = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphabetic() {
                attr.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match attr.as_str() {
            "weak" => Token::WeakAttr,
            "sync" => Token::SyncAttr,
            "own" => Token::OwnAttr,
            "actor" => Token::ActorAttr,
            _ => Token::Invalid('#'),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(c) = self.current_char {
            match c {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Token::String(string);
                }
                '\\' => {
                    self.advance();
                    if let Some(next) = self.current_char {
                        string.push(match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '"' => '"',
                            '\\' => '\\',
                            _ => next,
                        });
                        self.advance();
                    }
                }
                _ => {
                    string.push(c);
                    self.advance();
                }
            }
        }
        Token::Invalid('"') // Unterminated string
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::Eof,
            Some(c) => {
                if self.position > self.input.len() * 2 {
                    return Token::Eof;
                }

                match c {
                    '0'..='9' => self.read_number(),
                    'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                    '#' => self.read_attribute(),
                    '"' => self.read_string(),
                    '~' => {
                        self.advance();
                        Token::Tilde
                    }
                    '@' => {
                        self.advance();
                        Token::At
                    }
                    '+' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::PlusEq
                        } else if self.current_char == Some('+') {
                            self.advance();
                            Token::PlusPlus
                        } else {
                            Token::Plus
                        }
                    }
                    '-' => {
                        self.advance();
                        if self.current_char == Some('>') {
                            self.advance();
                            Token::Arrow
                        } else if self.current_char == Some('=') {
                            self.advance();
                            Token::MinusEq
                        } else if self.current_char == Some('-') {
                            self.advance();
                            Token::MinusMinus
                        } else {
                            Token::Minus
                        }
                    }
                    '*' => {
                        self.advance();
                        Token::Multiply
                    }
                    '/' => {
                        self.advance();
                        Token::Divide
                    }
                    '=' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::Eq
                        } else {
                            Token::Assign
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::NotEq
                        } else {
                            Token::Not
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::LtEq
                        } else {
                            Token::Lt
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Token::GtEq
                        } else {
                            Token::Gt
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.current_char == Some('&') {
                            self.advance();
                            Token::And
                        } else {
                            Token::Invalid('&')
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.current_char == Some('|') {
                            self.advance();
                            Token::Or
                        } else {
                            Token::Invalid('|')
                        }
                    }
                    ':' => {
                        self.advance();
                        if self.current_char == Some(':') {
                            self.advance();
                            Token::DoubleColon
                        } else {
                            Token::Colon
                        }
                    }
                    '(' => {
                        self.advance();
                        Token::LParen
                    }
                    ')' => {
                        self.advance();
                        Token::RParen
                    }
                    '{' => {
                        self.advance();
                        Token::LBrace
                    }
                    '}' => {
                        self.advance();
                        Token::RBrace
                    }
                    '[' => {
                        self.advance();
                        Token::LBracket
                    }
                    ']' => {
                        self.advance();
                        Token::RBracket
                    }
                    ',' => {
                        self.advance();
                        Token::Comma
                    }
                    '.' => {
                        self.advance();
                        Token::Dot
                    }
                    ';' => {
                        self.advance();
                        Token::Semicolon
                    }
                    '%' => {
                        self.advance();
                        Token::Modulus
                    }
                    _ => {
                        let invalid = c;
                        self.advance();
                        Token::Invalid(invalid)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("let x: i32 = 42;".to_string());

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::TypeI32);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Integer(42));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_ownership_and_attributes() {
        let mut lexer = Lexer::new("#sync struct Data { value: ~String }".to_string());

        assert_eq!(lexer.next_token(), Token::SyncAttr);
        assert_eq!(lexer.next_token(), Token::Struct);
        assert_eq!(lexer.next_token(), Token::Identifier("Data".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Identifier("value".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Tilde);
        assert_eq!(lexer.next_token(), Token::Identifier("String".to_string()));
        assert_eq!(lexer.next_token(), Token::RBrace);
    }
}

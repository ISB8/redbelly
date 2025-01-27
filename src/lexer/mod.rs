use crate::RedbellyError;
use std::{collections::HashMap, fmt::Display};

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String(String), Number(f64),

  // Keywords.
  And, Class, Else, False, Func, For, If, Nil, Or,
  Print, Return, Super, This, True, Let, While,

  Eof
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::String(val) => write!(f, "{}", val),
            TokenType::Number(val) => write!(f, "{}", val),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme: String::from(lexeme),
            line
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    index: usize,
    line: usize,
    start: usize,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            index: 0,
            line: 1,
            start: 0,
            keywords: get_keywords(),
        }
    }

    /// Loops over every token in source, and scans the token. Once a given token has been scaned start is set to current.
    /// Anytime self.consume() is called outside this function it increases the size of the lexeme.
    pub fn scan(&mut self) -> Result<Vec<Token>, RedbellyError> {
        while !self.is_at_end() {
            self.start = self.index;
            match self.scan_token() {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));

        return Ok(self.tokens.clone());
    }

    /// Returns the character at the current index, then increments the current index. Panics if self.index is out of bounds
    fn consume(&mut self) -> &char {
        let c = self.source.get(self.index).expect("Out of Bounds");
        self.index += 1;
        return c;
    }

    /// Looks at current index without consuming it. Returns none if the index is out of bounds.
    fn peek(&self) -> Option<&char> {
        self.source.get(self.index)
    }

    /// Looks at index + 1 without consuming it. Returns none if the index is out of bounds
    fn peek_next(&self) -> Option<&char> {
        self.source.get(self.index + 1)
    }

    /// Pushes a token onto self.tokens. Gets the lexeme of the token by getting a slice from contents.
    /// The bounds of the slice are determined by the the range between start and current. See scan() for
    /// details on start
    fn add_token(&mut self, token_type: TokenType) -> Result<(), RedbellyError> {
        let lexeme: String = self.source[self.start..self.index].iter().clone().collect();
        self.tokens.push(Token::new(token_type, lexeme, self.line));
        return Ok(());
    }

    fn scan_token(&mut self) -> Result<(), RedbellyError> {
        let c: &char = self.consume();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.consume_and_add_token_or('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.consume_and_add_token_or('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.consume_and_add_token_or('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.consume_and_add_token_or('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => self.handle_slash(),
            '"' => self.handle_string(),
            // Ignore Whitespace
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => {
                if is_num(c) {
                    self.handle_numbers()
                } else if is_alpha(c){
                    self.handle_identifiers()
                } else {
                    Err(RedbellyError::new("Unexpected Character", self.line))
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        return self.index >= self.source.len();
    }

    /// Checks if the next character matches 'expected', and if so consumes a character and returns it. Otherwise returns none
    fn consume_or(&mut self, expected: char) -> Option<&char> {
        if self.is_at_end() {
            return None;
        }
        match self.peek() {
            Some(c) => {
                if c == &expected {
                    Some(self.consume())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn consume_and_add_token_or(
        &mut self,
        expected: char,
        true_token: TokenType,
        false_token: TokenType,
    ) -> Result<(), RedbellyError> {
        match self.consume_or(expected) {
            Some(_) => self.add_token(true_token),
            None => self.add_token(false_token),
        }
    }

    fn handle_slash(&mut self) -> Result<(), RedbellyError> {
        match self.consume_or('/') {
            None => self.add_token(TokenType::Slash),
            Some(_) => {
                // If returns none due to array out of bounds, uses null ascii value
                while self.peek().unwrap_or(&'\n') != &'\n' {
                    let _ = self.consume();
                }
                Ok(())
            }
        }
    }

    // TODO: Support Escape Sequences
    fn handle_string(&mut self) -> Result<(), RedbellyError> {
        loop {
            let p = self.peek();
            match p {
                Some('"') => {
                    let _ = self.consume();
                    break;
                }
                Some('\n') => self.line += 1,
                None => return Err(RedbellyError::new("Untermintated String", self.line)),
                Some(_) => (),
            }
            self.consume();
        }
        // Trims the Quotes
        let val: String = self.source[self.start + 1..self.index - 1]
            .iter()
            .clone()
            .collect();
        self.add_token(TokenType::String(val))
    }

    // Consider supporting negative numbers as literals
    fn handle_numbers(&mut self) -> Result<(), RedbellyError> {
        self.consume_all_nums();

        // Check if has decible, if so, consumes it
        match (
            self.peek(),
            is_num(self.peek_next().unwrap_or(&'\0')),
        ) {
            (Some('.'), true) => {
                let _ = self.consume();
            }
            _ => (),
        }

        self.consume_all_nums();

        let s: String = self.source[self.start..self.index].iter().clone().collect();

        match s.parse::<f64>() {
            Ok(val) => self.add_token(TokenType::Number(val)),
            Err(_) => Err(RedbellyError::new(
                "Failed to cast Number Lexeme to f64",
                self.line,
            )),
        }
    }

    /// Consumes all characters that are numbers until it finds a character that is not a number.
    fn consume_all_nums(&mut self) {
        loop {
            match self.peek() {
                Some(c) => {
                    if is_num(c) {
                        self.consume();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    fn handle_identifiers(&mut self) -> Result<(), RedbellyError>  {
        loop {
            let c = self.peek();
            if c == None {
                break;
            } else {
                if is_alpha_numeric(c.expect("If value is invalid, it will already have been returned")) {
                    self.consume();
                } else {
                    break;
                }
            }
        }

        let text: String = self.source[self.start..self.index].iter().clone().collect();
        match self.keywords.get(&text) {
            Some(val) => self.add_token(val.clone()),
            None => self.add_token(TokenType::Identifier)
        }
    }
}

fn is_num(c: &char) -> bool {
    if c >= &'0' && c <= &'9' {
        return true;
    }
    false
}

fn is_alpha(c: &char) -> bool {
    return (c >= &'a' && c <= &'z') || (c >= &'A' && c <= &'Z') || c == &'_';
}

fn is_alpha_numeric(c: &char) -> bool {
    return is_alpha(c) || is_num(c);
}

fn get_keywords() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();
    map.insert("and".to_string(),    TokenType::And);
    map.insert("class".to_string(),  TokenType::Class);
    map.insert("else".to_string(),   TokenType::Else);
    map.insert("false".to_string(),  TokenType::False);
    map.insert("for".to_string(),    TokenType::For);
    map.insert("func".to_string(),    TokenType::Func);
    map.insert("if".to_string(),     TokenType::If);
    map.insert("nil".to_string(),    TokenType::Nil);
    map.insert("or".to_string(),     TokenType::Or);
    map.insert("print".to_string(),  TokenType::Print);
    map.insert("return".to_string(), TokenType::Return);
    map.insert("super".to_string(),  TokenType::Super);
    map.insert("this".to_string(),   TokenType::This);
    map.insert("true".to_string(),   TokenType::True);
    map.insert("let".to_string(),    TokenType::Let);
    map.insert("while".to_string(),  TokenType::While);

    map
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    impl Token {
        fn from_str(token_type: TokenType, lexeme: &str) -> Self {
            Self {
                token_type,
                lexeme: String::from(lexeme),
                line: 1
            }
        }
    }
    #[test]
    fn test_single_characters() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::LeftParen, "("),
            Token::from_str(TokenType::RightParen, ")"),
            Token::from_str(TokenType::LeftBrace, "{"),
            Token::from_str(TokenType::RightBrace, "}"),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("(){}");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_consume_and_add_or() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::BangEqual, "!="),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("!=");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::Bang, "!"),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("!");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_comments() {
        let test_tokens: Vec<Token> = vec![
            Token::new(TokenType::LessEqual, "<=".to_string(), 2),
            Token::new(TokenType::Eof, "".to_string(), 2),
        ];
        let string = String::from("// Comment which should be ignored\n<=");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_strings() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::String(String::from("Test")), "\"Test\""),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("\"Test\"");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_integers() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::Number(123.), "123"),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("123");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_floats() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::Number(693.8932), "693.8932"),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("693.8932");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }

    #[test]
    fn test_identifier() {
        let test_tokens: Vec<Token> = vec![
            Token::from_str(TokenType::Identifier, "foo"),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("foo");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }
}

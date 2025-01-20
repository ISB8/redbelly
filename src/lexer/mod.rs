use crate::RedbellyError;

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
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Func, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    fn from_str(token_type: TokenType, lexeme: &str) -> Self {
        Self {
            token_type,
            lexeme: String::from(lexeme),
        }
    }
    fn new(token_type: TokenType, lexeme: String) -> Self {
        Self {
            token_type,
            lexeme: String::from(lexeme),
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    index: usize,
    line: usize,
    start: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            index: 0,
            line: 1,
            start: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, RedbellyError> {
        while !self.is_at_end() {
            self.start = self.index;
            match self.scan_token() {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        self.tokens.push(Token::from_str(TokenType::Eof, ""));

        return Ok(self.tokens.clone());
    }

    fn consume(&mut self) -> &char {
        let c = self.source.get(self.index).expect("Out of Bounds");
        self.index += 1;
        return c;
    }

    /// Looks one index ahead. Returns none if index + 1 would escape the bounds of the vector
    fn peek(&self) -> Option<&char> {
        self.source.get(self.index)
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), RedbellyError> {
        let lexeme: String = self.source[self.start..self.index].iter().clone().collect();
        self.tokens.push(Token::new(token_type, lexeme));
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
            // Ignore Whitespace
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => return Err(RedbellyError::new("Unexpected Character", self.line)),
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
            Some(_) => {
                self.add_token(true_token)
            }
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
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

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
            Token::from_str(TokenType::LessEqual, "<="),
            Token::from_str(TokenType::Eof, ""),
        ];
        let string = String::from("// Comment which should be ignored\n<=");
        let mut lexer = Lexer::new(string.clone());
        let tokens = lexer.scan().unwrap();
        assert_eq!(test_tokens, tokens);
    }
}

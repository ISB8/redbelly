use crate::RedbellyError;



#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum TokenType {
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

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
    // Using debug print here is a bit stinky
    fn to_string(&self) -> String {
        return format!("{:?} {}", self.token_type, self.lexeme);
    }
}

pub struct Lexer {
    source: Vec<u8>,
    tokens: Vec<Token>,
}

impl Lexer {

    pub fn new() {
        todo!();
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), RedbellyError> {
        todo!();
    }

    fn scan_token(&mut self) -> Result<(), RedbellyError> {
        let c: char = todo!();
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
            _ => return Err(RedbellyError::new("Unexpected Character", 0)),
        }
    }
}


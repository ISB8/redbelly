use std::{
    fmt::format, fs::File, io::{stdin, stdout, Read, Write}
};

struct RedbellySession {
    has_err: bool,
}

impl RedbellySession {
    fn new() -> Self {
        Self { has_err: true }
    }

    fn run(&self, contents: String) {}

    fn error(&mut self, line: i32, message: String) {
        self.report(line, "".to_owned(), message);
    }

    fn report(&mut self, line: i32, location: String, message: String) {
        eprintln!("[Line {}] Error {}: {}", line, location, message);
        self.has_err = true;
    }
}

pub fn run_file(file_path: &String) -> Result<(), ()> {
    let mut contents = String::new();
    let file = File::open(file_path)
        .expect("Invalid File Path")
        .read_to_string(&mut contents);
    let session = RedbellySession::new();
    session.run(contents);
    if session.has_err {
        return Err(());
    } else {
        return Ok(());
    }
}

pub fn run_prompt() {
    let mut session = RedbellySession::new();
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => session.run(input),
            Err(error) => print!("Error Reading Input: {}", error),
        };
        session.has_err = false;
    }
}

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolen, Slash, Star,

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

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: i32) -> Self{
        Self {
            token_type,
            lexeme,
            line,
        }
    }
    // Using debug print here is a bit stinky
    fn to_string(&self) -> String {
        return format!("{:?} {}", self.token_type, self.lexeme)
    }
}

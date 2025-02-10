use lexer::Lexer;
use parser::{environment::Environment, interpret, Parser};
use std::{
    fmt::{self, Display},
    fs::File,
    io::{stdin, stdout, Read, Write},
};

fn run(contents: String, environment: &mut Environment) {
    let mut lexer = Lexer::new(contents);
    let result = lexer.scan();
    match result {
        Ok(tokens) => {
            let result = Parser::parse_tokens(tokens);
            match result {
                Ok(statements) => {
                    if let Some(err) = interpret(statements, environment) {
                        eprintln!("{}", err)
                    }
                }
                Err(error) => {
                    for err in error {
                        eprintln!("{}", err)
                    }
                }
            }
        }
        Err(error) => eprintln!("{:?}", error),
    }
}

pub fn run_file(file_path: &String) {
    let mut contents = String::new();
    let _ = File::open(file_path)
        .expect("Invalid File Path")
        .read_to_string(&mut contents)
        .expect("Invalid File Contents");
    run(contents, &mut Environment::new(None));
    // Proper exit codes one day
}

pub fn run_prompt() {
    let mut environment = Environment::new(None);
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => eprintln!("Error Reading Input: {}", error),
        };
        if input == *"exit()\n" {
            return;
        } else {
            run(input, &mut environment);
        }
    }
}

#[derive(Debug)]
struct RedbellyError {
    message: String,
    line: usize,
}

impl RedbellyError {
    fn new(message: &str, line: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
        }
    }
}

impl Display for RedbellyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Line {}]: {}", self.line, self.message)
    }
}

mod lexer;
mod parser;
mod redbelly_value;

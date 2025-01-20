use lexer::Lexer;
use std::{
    fmt::{self, Display}, fs::File, io::{stdin, stdout, Read, Write}
};

fn run(contents: String) {
    let mut lexer = Lexer::new(contents);
    let result = lexer.scan();
    match result {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token)
            }
        }
        Err(error) => eprintln!("{:?}", error)
    }
}

pub fn run_file(file_path: &String) {
    let mut contents = String::new();
    let _ = File::open(file_path)
        .expect("Invalid File Path")
        .read_to_string(&mut contents)
        .expect("Invalid File Contents");
    run(contents);
    // Proper exit codes one day
}

pub fn run_prompt() {
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut input = String::new();
        match stdin().read_to_string(&mut input) {
            Ok(_) => run(input),
            Err(error) => eprintln!("Error Reading Input: {}", error),
        };
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

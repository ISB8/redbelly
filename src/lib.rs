use lexer::Lexer;
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

fn run(contents: Vec<u8>) -> Result<(), RedbellyError> {
    todo!()
}

pub fn run_file(file_path: &String) {
    let mut contents: Vec<u8> = vec![];
    let _ = File::open(file_path)
        .expect("Invalid File Path")
        .read_to_end(&mut contents)
        .expect("Invalid File Contents");
    run(contents);
    // Proper exit codes one day
}

pub fn run_prompt() {
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut input = vec![];
        match stdin().read_to_end(&mut input) {
            Ok(n) => run(input).unwrap(),
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

mod lexer;

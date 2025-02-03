use std::env;
use redbelly::{run_file, run_prompt};


// Todo: Return proper exit codes (Something to do with returning results)
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        std::cmp::Ordering::Less => run_prompt(),
        std::cmp::Ordering::Equal => run_file(&args[1]),
        std::cmp::Ordering::Greater => println!("Usage: redbelly [script]"),
    }
}
use std::env;
use redbelly::{run_file, run_prompt};


// Todo: Return proper exit codes (Something to do with returning results)
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: redbelly [script]")
    } else if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        run_prompt();
    }
}
use std::env;
use std::process::exit;

mod rlox;

fn main() {


    let mut rlox:rlox::Rlox = rlox::Rlox::new();



    let args: Vec<String> = env::args().skip(1).collect();
    // differs from the book here b/c in Java args[] has only the strings after the file name. In Rust, the first 
    // el. in the args array is the filename
    if args.len() > 1{
        println!("Usage: Rlox v0.0.1");

        exit(0);
    } else if args.len() == 1 {
        rlox.run_file(&args[0]);
    } else {
        rlox.run_prompt();
    }
}

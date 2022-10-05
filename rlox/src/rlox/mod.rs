use std::env;
use std::process::exit;
use std::fs;
use std::io;
use std::io::Write;

mod scanner;


pub struct Rlox {
    had_error:bool,
}


// #[derive(Debug, Display)]



impl Rlox{
    pub fn new()->Rlox{
        Rlox{
            had_error:true
        }
    }

    pub fn run_file(&self,path: &String){
        let contents = fs::read(path).ok().expect("Invalid file");
        self.run(&String::from_utf8(contents).expect("Failed"));

        if self.had_error{
            exit(65);
        }
    }

    


    pub fn run(&self,source: &String){
        let mut scanner:scanner::Scanner = scanner::Scanner::new(source);
        let tokens:Vec<scanner::Token> = scanner.get_tokens();

        for token in tokens{
            println!("{}", token);
        }


    }

    pub fn run_prompt(&mut self){
        loop{
            print!(">> ");
            io::stdout().flush().expect("Couldn't print prompt");
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line");
            println!("{}", line);
            self.run(&line);
            self.had_error = false;
        }
    }

    pub fn report(&mut self,line: usize, location: String, message: String){
        eprintln!("[{}] Error {}: {}", line, location, message);
        self.had_error=true;
    }

    fn error(&mut self,line: usize, message: String){
        self.report(line, "".to_string(),message);

    }


}
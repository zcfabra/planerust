use std::env;
use std::process::exit;
use std::fs;
use std::io;
use std::io::Write;
// Scanning




struct Scanner{

}


impl Scanner{
    fn new(source: &String)->Scanner{
        Scanner{}
    }
}

struct Rlox {
    had_error:bool,
}



impl Rlox{
    fn new()->Rlox{
        Rlox{
            had_error:true
        }
    }

    fn run_file(&self,path: &String){
        let contents = fs::read(path).ok().expect("Invalid file");
        println!("{:?}", contents);
    }

    


    fn run(&self,source: &String){
        let scanner:Scanner = Scanner::new(source);


    }

    fn run_prompt(&mut self){
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

    pub fn report(&self,line: usize, location: String, message: String){
        eprintln!("[{}] Error {}: {}", line, location, message);
    }

    fn error(&self,line: usize, message: String){
        self.report(line, "".to_string(),message);

    }


}




    

    
    // Errors 
   



fn main() {


    let mut rlox: Rlox = Rlox::new();



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

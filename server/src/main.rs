

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration
};

use std::fs;

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./assets/test.json")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./assets/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let content_length = content.len();

    let res = format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{content}");
    stream.write_all(res.as_bytes()).unwrap();
    
}

static  MAX_THREADS:usize = 10;

struct ThreadPool{
    num_threads: usize,
    // threads: Vec<thread::Thread>,

}

impl ThreadPool{
    fn new(num_threads:usize)->ThreadPool{
        assert!(num_threads>0);
        ThreadPool{num_threads: num_threads}
    }

    fn execute<F>(&self, f: F )
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

fn main(){


    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(MAX_THREADS);
    println!("{:?}", std::env::current_dir().expect("hi"));
    for stream in listener.incoming(){ 
        
        // incoming method returns an iterator that yields a seq. of TcpStreams,
        // a stream is a connection between a client and server 
        let stream = stream.unwrap();

        thread::spawn(||{
            handle_connection(stream)
        });

        // println!("connection established with {}", stream.peer_addr().unwrap());

    }
}
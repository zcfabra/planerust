

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self},

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
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>

}

impl ThreadPool{
    fn new(num_threads:usize)->ThreadPool{
        assert!(num_threads>0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{workers, sender:Some(sender)}
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job)=>{
                    println!("Worker {} got job; executing", id);
                    job();
                }
                Err(_)=>{
                    println!("Worker {} dissconnected, shutting down", id);
                    break;
                }
            }
        });
        Worker {id, thread:Some(thread)}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main(){


    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(MAX_THREADS);
    println!("{:?}", std::env::current_dir().expect("hi"));
    for stream in listener.incoming(){ 
        
        // incoming method returns an iterator that yields a seq. of TcpStreams,
        // a stream is a connection between a client and server 
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });

        // println!("connection established with {}", stream.peer_addr().unwrap());

    }
}
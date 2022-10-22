use std::net::SocketAddr;

use tokio::{fs, stream, sync::broadcast};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

fn get_default<T>()-> T
where
    T: Default,
{
    Default::default()
}

#[tokio::main]
async fn main() {
    // let value = get_default::<i32>();

    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    let (tx,  rx) = broadcast::channel::<(String, SocketAddr)>(10);

    

    loop {

        let (mut stream, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        
        tokio::spawn(async move {
            let (rd, mut wr) = io::split(stream);
            let mut reader = BufReader::new(rd);
            let mut line = String::new();

            loop {
                tokio::select! {
                    // result = reader.read_line(&mut line)=>{
                    //     if result.unwrap() == 0 {
                    //         break;
                    //     }
                        
                    //     tx.send((line.clone(), addr)).unwrap();
                    //     line.clear();
                    // }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            wr.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

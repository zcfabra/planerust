use tokio::{fs, stream};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    let (mut stream, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];

        let n = stream.read(&mut buffer).await.unwrap();
        
        
        stream.write_all(&buffer[0..n]).await.unwrap();
        
    }
}

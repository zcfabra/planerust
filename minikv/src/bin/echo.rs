

use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::fs::File;
use tokio::net::{TcpStream, TcpListener};



#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = io::split(stream);

            
        });
    }

    Ok(())

}
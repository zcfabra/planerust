use tokio::{net::TcpStream, io::BufWriter};

use bytes::{Bytes, BytesMut};
use mini_redis::{Frame, Result};

// enum Frame{
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>,)
// }

struct Connection{
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream)->Connection{
        Connection{
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4*1024),
        }
    }

    



}

fn main(){

}


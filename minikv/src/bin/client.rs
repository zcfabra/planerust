use mini_redis::client;
use tokio::sync::mpsc;
use bytes::Bytes;

#[derive(Debug)]
enum Command{
    Get{
        key: String,
    },
    Set{
        key: String, 
        val: Bytes,
    }
}

#[tokio::main]
async fn main(){

    let (tx, mut rx) = mpsc::channel(32);

    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let t1 = tokio::spawn(async {
        let res = client.get("hello").await;
    });

    let t2 = tokio::spawn(async{
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
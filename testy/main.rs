use std::thread;

use std::sync::{Mutex, Arc};
static  NUM_THREADS:i32 = 10;


fn compute (num: i128){
    let computed = num.pow(2);
    println!("{}", computed);

}
fn main(){
    let  vec: Arc<Mutex<Vec<i128>>> = Arc::from(Mutex::from(Vec::from([1231241231, 12312412,123124124124,124124])));
    let mut threads= Vec::new();


    for _ in 0..NUM_THREADS{
        let vec = vec.clone();
        threads.push(thread::spawn(move ||{
            let mut x = vec.lock().expect("Error on acquiring lock");
            let hi = x.pop();
            match hi {
                Some(i)=>compute(i),
                None=>return
            }

            
        }))
    }

    for thread in threads{
        _ = thread.join();
    }
}
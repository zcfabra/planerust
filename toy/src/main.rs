static B: [u8; 10] = [99,97,114,114,121,116,111,119,101,108];
static C: [u8; 11] = [116,104,97,110,107,115,102,105,115,104,0];

fn main(){
    let a = 42;
    let b = &B;
    let c = &C;
    let d = &a;
    let e = &a;

    println!("{}", e);

    println!("a: {} b: {:p} c: {:p} d:{:p}", a,b,c,d);
    println!("{}", e);

}
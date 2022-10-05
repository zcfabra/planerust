fn main(){
    // isolating the sign bit
    // let n:f32 = -42.42;
    // let b: u32 = n.to_bits();

    // let sign = b >> 31;

    // println!("{}", sign);


    // isolating the exponent

    let n:f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let exp = n_bits >> 23;
    let exp = exp & 0xff;
    let exp = (exp as i32) - 127;

    
}
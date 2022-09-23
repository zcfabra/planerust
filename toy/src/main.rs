fn main() {

    let mut fun:Vec<i32> = Vec::from([1,2,3,4,5,6,7,8]);

    for each in fun.iter_mut(){
        *each = each.pow(2);
    }

    println!("{:?}", fun);
}

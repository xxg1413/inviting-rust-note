#![allow(unused)]

use std::collections::HashMap;


fn add_one(i: &mut u32) {

    *i += 1;
}


fn plus_one(i:& u32) -> u32 {
    let i = i + 1;
    i
}


fn main() {

    let mut a = 41;
    add_one(&mut a);
    println!("{:?}", a);

    let mut a = 41;
    let b = plus_one(&a);
    println!("{:?}",b);

    let mut h = HashMap::new();
    h.insert("anwser", 42);
    println!("anwser is {:?}", h["anwser"]);
}
#![allow(unused)]

fn main() {
    
    let a = 42;
    let b = a;
    println!("{:?}", a);


    let a = "42";
    let b = a;
    println!("{:?}", a);


    let a = "42".to_string();
    let b : &str = &a;

    println!("{:?}",b);



    let a = "42".to_string();
    let b : &str = &a;
    let c = b;
    //println!("{:?}",b);  //borrowed 
    println!("{:?}",c); 
}
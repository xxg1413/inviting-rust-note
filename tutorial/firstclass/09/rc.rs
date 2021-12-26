use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
    println!("{}", a);

    let b = a.clone();
    println!("{}", b);

    let c = a.clone();
    println!("{}", c);


}
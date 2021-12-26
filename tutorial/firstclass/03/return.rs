fn pi() -> f64 {
    3.141592653589793
}


fn not_pi()  {
    3.141592653589793;
    // return unit
}


fn main() {
    let pi = pi();
    let not_pi = not_pi();

    println!("pi: {:?}\nnot_pi:{:?}", pi, not_pi);
}


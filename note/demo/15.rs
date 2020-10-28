fn main() {

    let x: Box<i32> = Box::new(42);
    let y = *x;

    assert_eq!(y, 42);
}
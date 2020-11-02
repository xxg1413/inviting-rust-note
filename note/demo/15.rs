
use std::ops::Deref;

struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}


impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn main() {

    let x: Box<i32> = Box::new(42);
    let y = *x;

    assert_eq!(y, 42);

    let x = 5;
    let y = MySmartPointer::new(x);
    
    assert_eq!(5, *y);
}
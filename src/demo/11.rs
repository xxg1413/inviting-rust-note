
fn counter(i: i32) -> impl FnMut(i32) ->i32 {
    move |n| n+i 
}


fn main() {
    let mut f = counter(2);
    println!("{:?}", f(1));
}
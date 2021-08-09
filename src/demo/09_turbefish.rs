
fn foo<T>(x: T) -> T {
    println!("{}", std::any::type_name::<T>());
    return x;
}

fn main() {
    println!("{:?}",foo(1)); //i32
    println!("{:?}",foo("1"));
    println!("{:?}",foo::<u32>(1)); //u32
}
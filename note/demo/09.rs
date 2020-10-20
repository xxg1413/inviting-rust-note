#[repr(C)] //指定内存布局属性，编译器不会重排结构

struct A {
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    println!("{:?}",std::mem::size_of::<A>());
    // 重排： 8  不重排: 12 
    let v = A {a: 1, b: 2, c: 3};
   
}
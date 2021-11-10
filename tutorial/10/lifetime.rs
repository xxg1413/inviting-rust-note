fn main() {

    let x: u32;

    {
        let y = 42;
        let x = &y;

    }

    println!("x:{}", x);
}
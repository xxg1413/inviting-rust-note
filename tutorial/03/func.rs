fn apply(value: i32, f: fn(i32) -> i32) -> i32 {

    f(value)

}

fn square(value: i32) -> i32 {

    value * value

}

fn cube(value: i32) -> i32 {

    value * value * value

}

fn main() {
    println!("apply square:{}", apply(2,square));
    println!("apply cuba:{}", apply(2,cube));

}
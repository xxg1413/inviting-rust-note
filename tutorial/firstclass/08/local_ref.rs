fn main() {

    let a = local_ref();
    println!("{}", a);
}

fn local_ref<`a>() -> &`a i32 {
    let a = 1;
    &a
}
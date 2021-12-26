fn main() {

    let s = "hello world".to_string();

    println!("地址：ss: {:p}\n,s:{:p}, len: {}, capacity: {}, size: {}",
                         &"hello world", &s, s.len(), s.capacity(), std::mem::size_of_val(&s));
}
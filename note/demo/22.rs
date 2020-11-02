fn main() {

    let v: Vec<i32> = vec![1,2,3].into_iter().map(|i| i+1).rev().collect();
    println!("{:?}", v);
}
fn sum(data: &Vec<u32>) -> u32 {

    data.iter().fold(0, |acc, x| acc + x)
}

fn main() {

    let data = vec![1, 2, 3, 4, 5];
    let data1 = &data;

    let result1 = sum(data1);
    println!("{}", result1);
}
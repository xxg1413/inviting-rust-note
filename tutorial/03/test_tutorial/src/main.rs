
pub fn sum(a:i32, b:i32) -> i32 {
    a + b 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 2), 3);
    }
}


fn main() {

    let sum = sum(1, 2);
    println!("{}", sum);

}

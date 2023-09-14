fn main() {
    println!("Hello, world! ouiii");
    println!("5 + 3 = {}", addition(5, 3));
    println!("5 - 3 = {}", subtraction(5, 3));
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(addition(1, 1), 2);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(5, 3), 2);
    }
}

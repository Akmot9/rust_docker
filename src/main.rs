use serde::{self, Serialize, Deserialize};
use std::io::{self, Read};
use libflate::gzip::Decoder;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    a: i32,
    b: i32,
    c: i32,
}


fn main() {
    let encoded_data = [31, 139, 8, 0, 123, 0, 0, 0, 0, 3, 1, 12, 0, 243, 255,
    72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,
    163, 28, 41, 28, 12, 0, 0, 0];

    let mut decoder = Decoder::new(&encoded_data[..]).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();

    assert_eq!(buf, b"Hello World!");

    println!("Hello, world! ouiii");
    println!("5 + 3 = {}", addition(5, 3));
    println!("5 - 3 = {}", subtraction(5, 3));
    division(2,2);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
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

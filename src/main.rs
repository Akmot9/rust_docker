use libflate::gzip::{Decoder, Encoder};
use serde::{self, Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    a: i32,
    b: i32,
    c: i32,
}

fn compress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = Encoder::new(Vec::new())?;
    encoder.write_all(data)?;
    let compressed_data = encoder.finish().into_result()?;
    Ok(compressed_data)
}

fn decompress(encoded_data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = Decoder::new(encoded_data)?;
    let mut buf: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression() {
        let original_data = b"Hello, world!";
        let compressed_data = compress(original_data).unwrap();
        let decompressed_data = decompress(&compressed_data).unwrap();

        assert_eq!(original_data.to_vec(), decompressed_data);
    }

    #[test]
    fn test_addition() {
        assert_eq!(addition(1, 1), 2);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtraction(5, 3), 2);
    }
}

fn main() {
    println!("Hello, world! ouiii");
    println!("5 + 3 = {}", addition(5, 3));
    println!("5 - 3 = {}", subtraction(5, 3));
    println!("2 / 2 = {}", division(2, 2));

    let original_data = b"Hello, world!";
    let compressed_data = compress(original_data).unwrap();
    let decompressed_data = decompress(&compressed_data).unwrap();
    println!("{:?}", decompressed_data);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

fn division(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0;
    }
    a / b
}

fn main() {
    println!("Hello, world! ouiii");
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_main() {
        // Votre logique de test ici.
        assert_eq!(1, 1);  // Cette assertion vérifie si 1 est égal à 1
    }
}

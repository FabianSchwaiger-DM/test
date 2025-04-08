fn main() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // This allows access to the functions in the parent module

    #[test]
    fn test_add() {
        // Test the add function
        assert_eq!(add(2, 3), 5); // Expecting 2 + 3 = 5
        assert_eq!(add(-1, 1), 0); // Expecting -1 + 1 = 0
        assert_eq!(add(0, 0), 0);  // Expecting 0 + 0 = 0
    }
}

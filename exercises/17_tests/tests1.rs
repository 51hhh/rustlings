fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;  // Import everything from the outer module

    #[test]
    fn you_can_assert() {
        // Test even number
        assert!(is_even(4));
        
        // Test odd number
        assert!(!is_even(5));
    }
}
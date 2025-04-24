// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // Test basic cases
        assert_eq!(power_of_2(0), 1);  // 2^0 = 1
        assert_eq!(power_of_2(1), 2);  // 2^1 = 2
        assert_eq!(power_of_2(4), 16); // 2^4 = 16
        
        // Test edge case (maximum u8 value would panic due to overflow)
        assert_eq!(power_of_2(63), 1 << 63); // 2^63
    }
}
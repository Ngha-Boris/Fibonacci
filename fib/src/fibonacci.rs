/// Calculates the nth Fibonacci number
///
/// # Arguments
/// * `n` - The position in the Fibonacci sequence (0-indexed)
///
/// # Returns
/// The Fibonacci number at position n
///
/// # Examples
/// ```
/// assert_eq!(fib::fibonacci::fibonacci(0), 0);
/// assert_eq!(fib::fibonacci::fibonacci(1), 1);
/// assert_eq!(fib::fibonacci::fibonacci(5), 5);
/// ```
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_five() {
        assert_eq!(fibonacci(5), 5);
    }

    #[test]
    fn test_fibonacci_ten() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fibonacci_twenty() {
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Test the first several numbers in the sequence
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, &value) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u64), value);
        }
    }

    #[test]
    fn test_fibonacci_larger_value() {
        // Test a larger value within reasonable compute range
        assert_eq!(fibonacci(30), 832040);
    }
}

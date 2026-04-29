// Integration tests for the fibonacci library

use fib::fibonacci;

#[test]
fn test_fibonacci_api_basic() {
    // Test basic API access
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(5), 5);
}

#[test]
fn test_fibonacci_api_sequence() {
    // Verify the sequence property: fib(n) = fib(n-1) + fib(n-2)
    for n in 2..20 {
        let fib_n = fibonacci(n);
        let fib_n_minus_1 = fibonacci(n - 1);
        let fib_n_minus_2 = fibonacci(n - 2);
        assert_eq!(
            fib_n,
            fib_n_minus_1 + fib_n_minus_2,
            "Fibonacci sequence property failed at n={}",
            n
        );
    }
}

#[test]
fn test_fibonacci_api_edge_cases() {
    // Test that fibonacci handles the base cases correctly
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn test_fibonacci_api_consistency() {
    // Verify consistency: calling fibonacci multiple times should return the same result
    for n in [0, 1, 5, 10, 15].iter() {
        let result1 = fibonacci(*n);
        let result2 = fibonacci(*n);
        assert_eq!(result1, result2, "Inconsistent results for n={}", n);
    }
}

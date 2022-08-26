use weblab::weblab;

#[weblab(programming_assignment)]
/// # Even or Odd
///
/// Create a function is_even, with parameters
///
/// - n, of type i64
/// - return type: bool
///
/// The function should return true when n is even and false when n is odd.
///
/// # Absolute value
///
/// Create a function absolute, with parameters
///
/// - n, of type i64
/// - return type: i64
///
/// The function should return the absolute value of n.
#[weblab(title = "numeric properties")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn is_even(a: i64) -> bool {
            a % 2 == 0
        }

        pub fn absolute(n: i64) -> i64 {
            n.abs()
        }
    }

    #[weblab(solution_template)]
    mod solution_template {}
    #[weblab(test_template)]
    mod test_template {
        /// These imports may be unused if you don't have tests. The `allow` line
        /// simply makes it so the compiler won't give a warning about this.
        #[allow(unused_imports)]
        use super::solution::{absolute, is_even};
    }

    #[weblab(test)]
    mod test {
        use super::solution::{absolute, is_even};

        #[test]
        fn test_is_even_4() {
            assert!(is_even(4));
        }

        #[test]
        fn test_is_even_3() {
            assert!(!is_even(3));
        }

        #[test]
        fn test_absolute_pos() {
            assert_eq!(absolute(5), 5);
        }

        #[test]
        fn test_absolute_zero() {
            assert_eq!(absolute(0), 0);
        }

        #[test]
        fn test_absolute_neg() {
            assert_eq!(absolute(-3), 3);
        }
    }
}

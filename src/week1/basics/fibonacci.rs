use weblab::weblab;

#[weblab(programming_assignment)]
/// # Fibonacci
/// A popular mathematical sequence is the Fibonacci sequence.
/// The Fibonacci sequence is defined mathematically as follows for any non-negative integer $n$:
/// $$
/// F_0 = 0
/// F_1 = 1
/// F_n = F_{n-1} + F_{n-2}
/// $$
///
/// This means we can find the $n$-th Fibonacci number by adding the previous two.
/// E.g. we find that
/// $$
/// F_4 = 3
/// F_8 = 21
/// F_{12} = 144
/// $$
///
/// ## Assignment
/// Create a function `fibonacci(n: u64) -> u64;` that computes and returns the `n`th fibonacci number for any `n`.
#[weblab(title = "fibonacci")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn fibonacci(n: u64) -> u64 {
            if n == 0 {
                return 0;
            };

            let mut temp1;
            let mut temp2 = 0;
            let mut curr = 1;
            for _ in 2..=n {
                temp1 = temp2;
                temp2 = curr;

                curr = temp1 + temp2;
            }

            curr
        }
    }

    #[weblab(solution_template)]
    #[allow(unused_variables)]
    mod solution_template {
        use weblab::solution_only;

        pub fn fibonacci(n: u64) -> u64 {
            todo!()
        }
    }

    #[weblab(test_template)]
    mod test_template {
        use super::solution::fibonacci;

        #[test]
        fn test_fibonacci_4() {
            assert_eq!(fibonacci(4), 3);
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::fibonacci;
        use quickcheck::{Arbitrary, Gen};
        use quickcheck_macros::quickcheck;
        use rand::prelude::*;

        #[derive(Debug, Clone)]
        struct SmallNumber(u64);

        impl Arbitrary for SmallNumber {
            fn arbitrary(_: &mut Gen) -> SmallNumber {
                let mut rng = rand::thread_rng();
                SmallNumber(rng.gen_range(0..90))
            }
        }

        fn fibonacci_ref(n: u64) -> u64 {
            if n == 0 {
                return 0;
            };

            let mut temp1;
            let mut temp2 = 0;
            let mut curr = 1;
            for _ in 2..=n {
                temp1 = temp2;
                temp2 = curr;

                curr = temp1 + temp2;
            }

            curr
        }

        #[test]
        fn test_0() {
            assert_eq!(fibonacci(0), 0);
            assert_eq!(fibonacci(1), 1);
            assert_eq!(fibonacci(2), 1);
            assert_eq!(fibonacci(3), 2);
            assert_eq!(fibonacci(4), 3);
        }

        #[test]
        fn test_1() {
            assert_eq!(fibonacci(1), 1);
        }

        #[test]
        fn test_2() {
            assert_eq!(fibonacci(2), 1);
        }

        #[test]
        fn test_3() {
            assert_eq!(fibonacci(3), 2);
        }

        #[test]
        fn test_4() {
            assert_eq!(fibonacci(4), 3);
        }

        #[test]
        fn test_8() {
            assert_eq!(fibonacci(8), 21);
        }

        #[test]
        fn test_12() {
            assert_eq!(fibonacci(12), 144);
        }

        #[quickcheck]
        fn prop_fibonacci(n: SmallNumber) -> bool {
            let SmallNumber(n) = n;
            fibonacci(n) == fibonacci_ref(n)
        }
    }
}

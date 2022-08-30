use weblab::weblab;

#[weblab(programming_assignment)]
/// For each of the following assignments, you are asked to make a function. Make sure that
/// each function is defined publicly. In other words, your function signature will need to look
/// similar to what you can see below, notice the `pub`:
/// ```
/// pub fn some_name() { ... }
/// ```
///
/// Each function has the following signature:
///
/// - a of type i64
/// - b of type i64
/// - return type: i64
///
/// # Assignment
///
/// Create these functions according to the signature above:
///
/// * `add`, which adds a to b
/// * `subtract`, which subtracts a from b
/// * `multiply`, which multiplies a with b
/// * `divide`, which divides a by b
/// * `modulo`, which returns the remainder after division of a by b
#[weblab(title = "basic arithmetic")]
#[weblab(weight = 1)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn add(a: i64, b: i64) -> i64 {
            a + b
        }

        pub fn subtract(a: i64, b: i64) -> i64 {
            a - b
        }

        pub fn multiply(a: i64, b: i64) -> i64 {
            a * b
        }

        pub fn divide(a: i64, b: i64) -> i64 {
            a / b
        }

        pub fn modulo(a: i64, b: i64) -> i64 {
            a % b
        }
    }

    #[weblab(solution_template)]
    mod solution_template {}
    #[weblab(test_template)]
    mod test_template {
        use super::solution::add;

        /// feel free to make more tests for your code, like this. Especially for later weeks
        /// this will become necessary.
        #[test]
        fn test_add() {
            assert_eq!(add(3, 4), 7);
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::{add, divide, modulo, multiply, subtract};

        #[test]
        fn test_add() {
            assert_eq!(add(-10, 7), -3);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(subtract(-10, 7), -17);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(multiply(-10, 7), -70);
        }

        #[test]
        fn test_divide() {
            assert_eq!(divide(-10, 7), -1);
        }

        #[test]
        fn test_modulo() {
            assert_eq!(modulo(10, 7), 3);
        }
    }
}

use weblab::weblab;

#[weblab(programming_assignment)]
/// For each of the following assignments, you are asked to make a function. Make sure that
/// each function is defined publicly. In other words, your function signature will need to look
/// similar to, notice the `pub`:
/// ```
/// pub fn some_name() { ... }
/// ```
///
/// # Adding
///
/// Create a function add, with parameters
/// - a of type i64
/// - b of type i64
/// Return type: i64
///
/// The function should return the sum of a and b.
///
/// # Subtraction
///
/// Create a function subtract, with parameters
/// - a of type i64
/// - b of type i64
/// Return type: i64
///
/// The function should return the result of subtracting b from a.
///
/// # Multiplication
///
/// Create a function multiply, with parameters
/// - a of type i64
/// - b of type i64
/// Return type: i64
///
/// The function should return the multiplication of a and b.
///
/// # Division
///
/// Create a function divide, with parameters
/// - a of type i64
/// - b of type i64
/// Return type: i64
///
/// The function should return the result of dividing a by b.
/// Modulo
///
/// Create a function modulo, with parameters
/// - a of type i64
/// - b of type i64
/// Return type: i64
///
/// The function should return the remainder after division of a by b
#[weblab(title = "basic arithmetic")]
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

use weblab::weblab;

#[weblab(programming_assignment)]
/// For each of the following assignments, you are asked to implement a function related to references.
/// The comment above the function will explain what each function should do.
///
/// In this assignment, changing any of the signatures is not allowed.
#[weblab(title = "References")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        /// Return whether these number are equal.
        /// Note that both numbers are passed by value
        pub fn is_equal_v1(a: i64, b: i64) -> bool {
            a == b
        }

        /// Return whether these number are equal.
        /// Note that one number is passed by reference, the other is passed by value.
        pub fn is_equal_v2(a: &i64, b: i64) -> bool {
            *a == b
        }

        /// Return whether these number are equal.
        /// Note that both numbers are passed by reference.
        pub fn is_equal_v3(a: &i64, b: &i64) -> bool {
            *a == *b // or just a == b
        }

        /// Add the value of y to x.
        /// Note that x is passed by mutable reference, and y by reference.
        pub fn add_y_to_x(x: &mut i64, y: &i64) {
            *x += *y; // or just *x += y
        }

        /// Increment (add 1 to) x.
        /// Note that x is passed by mutable reference.
        pub fn increment_v1(x: &mut i64) {
            *x += 1;
        }

        /// Return x + 1.
        /// This MUST be implemented by calling the `increment_v1` function above
        pub fn increment_v2(x: i64) -> i64 {
            let mut x = x; // Changing `x` to `mut x` in the method signature is also allowed.
            increment_v1(&mut x);
            x
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        use weblab::{solution_only, template_only};

        /// Return whether these number are equal.
        /// Note that both numbers are passed by value
        pub fn is_equal_v1(a: i64, b: i64) -> bool {
            todo!()
        }

        /// Return whether these number are equal.
        /// Note that one number is passed by reference, the other is passed by value.
        pub fn is_equal_v2(a: &i64, b: i64) -> bool {
            todo!()
        }

        /// Return whether these number are equal.
        /// Note that both numbers are passed by reference.
        pub fn is_equal_v3(a: &i64, b: &i64) -> bool {
            todo!()
        }

        /// Add the value of y to x.
        /// Note that x is passed by mutable reference, and y by reference.
        pub fn add_y_to_x(x: &mut i64, y: &i64) {
            todo!()
        }

        /// Increment (add 1 to) x.
        /// Note that x is passed by mutable reference.
        pub fn increment_v1(x: &mut i64) {
            todo!()
        }

        /// Return x + 1.
        /// This MUST be implemented by calling the `increment` function above
        pub fn increment_v2(x: i64) -> i64 {
            todo!()
        }
    }

    #[weblab(test_template)]
    mod test_template {}

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_is_equal_v1() {
            assert!(is_equal_v1(1, 1));
            assert!(is_equal_v1(-18, -18));
            assert!(!is_equal_v1(-17, -18));
            assert!(!is_equal_v1(0, -1));
        }

        #[test]
        fn test_is_equal_v2() {
            assert!(is_equal_v2(&1, 1));
            assert!(is_equal_v2(&-18, -18));
            assert!(!is_equal_v2(&-17, -18));
            assert!(!is_equal_v2(&0, -1));
        }

        #[test]
        fn test_is_equal_v3() {
            assert!(is_equal_v3(&1, &1));
            assert!(is_equal_v3(&-18, &-18));
            assert!(!is_equal_v3(&-17, &-18));
            assert!(!is_equal_v3(&0, &-1));
        }

        #[test]
        fn test_add_y_to_x() {
            let mut x = 8;

            add_y_to_x(&mut x, &7);
            assert_eq!(x, 15);

            add_y_to_x(&mut x, &-18);
            assert_eq!(x, -3);

            add_y_to_x(&mut x, &999);
            assert_eq!(x, 996);
        }

        #[test]
        fn test_increment_v1() {
            let mut x = -3;

            increment_v1(&mut x);
            assert_eq!(x, -2);

            increment_v1(&mut x);
            assert_eq!(x, -1);

            x = 19;
            increment_v1(&mut x);
            assert_eq!(x, 20);

            increment_v1(&mut x);
            assert_eq!(x, 21);
        }

        #[test]
        fn test_increment_v2() {
            assert_eq!(increment_v2(100), 101);
            assert_eq!(increment_v2(-18), -17);
            assert_eq!(increment_v2(7319287), 7319288);
            assert_eq!(increment_v2(0), 1);
            assert_eq!(increment_v2(-1), 0);
        }
    }
}

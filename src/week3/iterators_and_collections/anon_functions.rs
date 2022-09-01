use weblab::weblab;

#[weblab(programming_assignment)]
/// Rust has something called a "closure". A closure is a function that you can save in a variable or pass to a function as a parameter. The function does not need a name,
/// and can be created inside other functions using the local variables of the enclosing function. Let's look at an example:
///
/// ```rs
/// let v = [1,2,3];
/// let w = v.map(|x| x + 5);
/// ```
///
/// The type of these functions is `impl Fn(...) -> ...`. They can be called using the normal function call notation.
/// If `f` is an anonymous function, `f(15)` calls `f`.
///
/// For this exercise, complete the body of the given functions.
///
#[weblab(title = "Anonymous Functions")]
#[weblab(weight = 1)]
mod assignment {
    #[weblab(solution)]
    #[allow(unused_parens)]
    mod solution {
        use weblab::{solution_only, template_only};

        /// This function takes no arguments and returns a function `u32 -> u32`.
        /// The function should add one to its argument.
        pub fn add_one() -> (impl Fn(u32) -> u32) {
            template_only! {
                todo!()
            }
            solution_only! {
                |n| n + 1
            }
        }

        /// This function takes as argument a tuple `(u32, u32)` and a function `u32 -> u32`.
        /// Apply the function to both elements of the tuple.
        ///
        /// This pattern is generally called a `map` function.
        pub fn map_tuple(pair: (u32, u32), f: impl Fn(u32) -> u32) -> (u32, u32) {
            template_only! {
                todo!()
            }
            solution_only! {
                (f(pair.0), f(pair.1))
            }
        }

        /// This function takes a tuple `(u32, u32)` as argument, and should return a tuple with both elements multiplied by 2.
        ///
        /// Use the `map_tuple` function you defined above to achieve this goal.
        pub fn double_tuple(pair: (u32, u32)) -> (u32, u32) {
            template_only! {
                todo!()
            }
            solution_only! {
                map_tuple(pair, |x| x * 2)
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        template_only! {
            #[test]
            fn test_map_tuple1() {
                assert_eq!(map_tuple((3, 5), |x| x - 1), (2, 4))
            }
        }

        solution_only! {
            #[test]
            fn test_add_one1() {
                let a1 = add_one();
                assert_eq!(a1(0), 1);
            }

            #[test]
            fn test_add_one2() {
                let a1 = add_one();
                assert_eq!(a1(4242), 4243);
            }

            #[test]
            fn test_add_one3() {
                let a1 = add_one();
                assert_eq!(a1(17), 18);
            }

            #[test]
            fn test_map_tuple2() {
                assert_eq!(map_tuple((8, 9), |x| x + 5), (13, 14));
            }

            #[test]
            fn test_map_tuple3() {
                assert_eq!(map_tuple((3, 5), |_| 42), (42, 42));
            }

            #[test]
            fn test_map_tuple4() {
                assert_eq!(map_tuple((0, 0), |x| x), (0, 0));
            }

            #[test]
            fn test_map_tuple5() {
                assert_eq!(map_tuple((22, 44), |x| x / 2), (11, 22));
            }

            #[test]
            fn test_double_tuple1() {
                assert_eq!(double_tuple((1, 2)), (2, 4));
            }

            #[test]
            fn test_double_tuple2() {
                assert_eq!(double_tuple((8, 8)), (16, 16));
            }

            #[test]
            fn test_double_tuple3() {
                assert_eq!(double_tuple((3, 7)), (6, 14));
            }

            #[test]
            fn test_double_tuple4() {
                assert_eq!(double_tuple((0, 0)), (0, 0));
            }

            #[test]
            fn test_double_tuple5() {
                assert_eq!(double_tuple((44, 22)), (88, 44));
            }
        }
    }
}

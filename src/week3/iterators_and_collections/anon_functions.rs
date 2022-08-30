use weblab::weblab;

#[weblab(programming_assignment)]
/// Rust has support for anonymous functions, that is, functions without a name.
/// These can be passed into other functions as an argument, or can even be returned from functions.
///
/// Anonymous functions can be constructed using the following notation: `|x| body`.
/// `x` is the name of the variable that the function takes, and `body` is the code that is executed using this variable.
///
/// The type of these functions is `fn(...) -> ...`. They can be called using the normal function call notation.
/// If `f` is an anonymous function, `f(15)` calls `f`.
///
/// For this exercise, complete the body of the given functions.
///
#[weblab(title = "Anonymous Functions")]
mod assignment {
    #[weblab(solution)]
    #[allow(unused_parens)]
    mod solution {
        use weblab::{solution_only, template_only};

        /// This function takes no arguments and returns a function `u32 -> u32`.
        /// The function should add one to its argument.
        pub fn add_one() -> (fn(u32) -> u32) {
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
        pub fn map_tuple(pair: (u32, u32), f: fn(u32) -> u32) -> (u32, u32) {
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
            fn test_map_tuple() {
                assert_eq!(map_tuple((3, 5), |x| x - 1), (2, 4))
            }
        }

        solution_only! {
            #[test]
            fn test_add_one() {
                let a1 = add_one();
                assert_eq!(a1(0), 1);
                assert_eq!(a1(17), 18);
                assert_eq!(a1(4242), 4243);
            }

            #[test]
            fn test_map_tuple() {
                assert_eq!(map_tuple((3, 5), |x| x - 1), (2, 4));
                assert_eq!(map_tuple((8, 9), |x| x + 5), (13, 14));
                assert_eq!(map_tuple((3, 5), |_| 42), (42, 42));
                assert_eq!(map_tuple((0, 0), |x| x), (0, 0));
                assert_eq!(map_tuple((22, 44), |x| x / 2), (11, 22));
            }

            #[test]
            fn test_double_tuple() {
                assert_eq!(double_tuple((1, 2)), (2, 4));
                assert_eq!(double_tuple((8, 8)), (16, 16));
                assert_eq!(double_tuple((3, 7)), (6, 14));
                assert_eq!(double_tuple((0, 0)), (0, 0));
                assert_eq!(double_tuple((44, 22)), (88, 44));
            }
        }
    }
}

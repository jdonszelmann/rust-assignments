use weblab::weblab;

#[weblab(programming_assignment)]
/// For this exercise, design the `Increment` trait.
/// It has two methods:
///
/// - `increment`, that should mutate the number and add 1 to it
/// - `decrement`, that should mutate the number and remove 1 from it
///
/// Finish the definition of the increment trait, and then implement it for `u64` and `i64`.
#[weblab(title = "Addable")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub trait Increment {
            fn increment(&mut self);
            fn decrement(&mut self);
        }

        impl Increment for u64 {
            fn increment(&mut self) {
                *self += 1;
            }

            fn decrement(&mut self) {
                *self -= 1;
            }
        }

        impl Increment for i64 {
            fn increment(&mut self) {
                *self += 1;
            }

            fn decrement(&mut self) {
                *self -= 1;
            }
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// TODO
        pub trait Increment {

        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_u64_incr() {
            let mut x = 17i64;
            x.increment();
            assert_eq!(x, 18);
        }

        #[test]
        fn test_i64_incr() {
            let mut x = 17i64;
            x.increment();
            assert_eq!(x, 18);
        }

        solution_only! {
            #[test]
            fn test_u64_decr() {
                let mut x = 17i64;
                x.decrement();
                assert_eq!(x, 16);
            }

            #[test]
            fn test_i64_decr() {
                let mut x = 17i64;
                x.decrement();
                assert_eq!(x, 16);
            }

            #[test]
            fn test_no_cheating() {
                fn is_increment(_: impl Increment) {

                }
                is_increment(4u64);
                is_increment(5i64);
            }
        }
    }
}

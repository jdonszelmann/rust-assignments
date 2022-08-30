use weblab::weblab;

#[weblab(programming_assignment)]
/// For this exercise, design the `Increment` trait.
/// It has two methods:
///
/// - `increment`, that should mutate the number and add 1 to it
/// - `decrement`, that should mutate the number and remove 1 from it
///
/// Neither of the methods returns a new value. Instead, they should take a mutable reference.
///
/// Finish the definition of the increment trait, and then implement it for `i64` and `f64`.
#[weblab(title = "Addable")]
#[weblab(weight=2)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub trait Increment {
            fn increment(&mut self);
            fn decrement(&mut self);
        }

        impl Increment for f64 {
            fn increment(&mut self) {
                *self += 1.0;
            }

            fn decrement(&mut self) {
                *self -= 1.0;
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
        pub trait Increment {}
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_i64_incr() {
            let mut x = 17;
            x.increment();
            assert_eq!(x, 18);
        }

        #[test]
        fn test_f64_incr() {
            let mut x = 17.0;
            x.increment();
            assert_eq!(x, 18.0);
        }

        solution_only! {
            fn almost_eq(a: f64, b: f64) -> bool {
                (a-b).abs() < 0.0000001
            }

            #[test]
            fn test_no_cheating() {
                fn is_increment(_: impl Increment) {

                }
                is_increment(4i64);
                is_increment(5f64);
            }

            #[test]
            fn test_i64_decr() {
                let mut x = 17i64;
                x.decrement();
                assert_eq!(x, 16);
            }

            #[test]
            fn test_f64_decr() {
                let mut x = 17.0;
                x.decrement();
                assert!(almost_eq(x, 16.0));
            }

            #[test]
            fn test_with_decimal() {
                let mut x = 17.5;
                x.decrement();
                assert!(almost_eq(x, 16.5));
            }
        }
    }
}

use weblab::weblab;

#[weblab(programming_assignment)]
/// Fixed point numbers are an alternative to floating point numbers for storing numbers with a fractional part.
/// The number has 32 bits for the whole numbers part and 32 bits for the fractional part.
/// The most significant bit of the fractional part has a value of `1/2`, the next a value of `1/4`, etc.
///
/// An implementation of this fixed point number is provided.
/// Implement the following traits:
///
/// - PartialEq and Eq, which provide the `eq` method to check if two fixed point numbers are equal.
///   The Eq trait signifies that the [equality laws](https://doc.rust-lang.org/std/cmp/trait.Eq.html) are satisfied.
/// - PartialCmp and Cmp, which provide the `cmp` method to check if two fixed point numbers are equal
/// - Clone, which provides a way to clone a fixed point number
/// - Copy, which means this type has Copy behaviour
/// - Add, which provides the 'add' method to add two fixed point numbers (you do NOT have to implement Sub, Mul, Div, Neg, and Rem).
///   Implementing this trait also allows the `a + b` notation can be used for fixed point numbers.
/// - Display, which provides a way to format a fixed point number for printing
/// - Default, which provides a default value for a fixed point number. The default value should be 0.
/// - Hash, which provides a way to Hash a fixed point number. This is necessary to use FixedPoints as the key in HashMaps and HashSets.
///
/// Note that some of these traits can be implemented by deriving the implementation.
/// This can save a lot of work in cases where the implementation is trivial.
/// For example, the `Clone` trait can be implemented by using `#[derive(Clone)]`, which just clones the components of the struct recursively.
#[weblab(title = "Fixed Point Numbers")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::solution_only;

        #[derive(PartialEq, Eq)]
        pub struct FixedPoint {
            whole: i32,
            fractional: u32,
        }


        solution_only! {

        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        template_only! {

        }

        solution_only! {

        }
    }
}

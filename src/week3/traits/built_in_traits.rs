use weblab::weblab;

#[weblab(programming_assignment)]
/// In the sign–and-magnitude representation, a signed number is represented by the bit pattern corresponding to the sign of the number for the sign bit
/// (the most significant bit, set to 0 for a positive number and to 1 for a negative number),
/// and the magnitude of the number (or absolute value) for the remaining bits.
///
/// A few examples of 32-bits sign-and-magnitude numbers:
///
/// `0 0000000000000000000000000000000 ->  0`
///
/// `0 0000000000000000000000000000111 ->  7`
///
/// `1 0000000000000000000000000000111 -> -7`
///
/// `1 0000000000000000000000000001001 -> -9`
///
/// `1 0000000000000000000000000000000 -> -0` (This number is considered equal to +0, even though its bits are different)
///
/// An implementation of sig-and-magnitude numbers is provided.
/// Implement the following traits:
///
/// - PartialEq and Eq. Make sure that the two zeroes are considered equal.
/// - PartialOrd and Ord.
/// - Clone and Copy.
/// - Add, which provides the 'add' method to add two sign-and-magnitude numbers (you do NOT have to implement Sub, Mul, Div, Neg, and Rem).
///   Implementing this trait also allows the `a + b` notation to be used.
/// - Display, which should display the sign-and-magnitude number. The format should be like `-123` for negative numbers and `123` for positive numbers.
///   Using the Display implementation of u32 may be useful.
/// - Debug, this may do anything you like. Use it to provide a useful debug representation for yourself.
/// - Default, which provides a default value. The default value should be +0.
///
/// Note that some of these traits can be implemented by deriving the implementation.
/// This can save a lot of work in cases where the implementation is trivial.
#[weblab(title = "Sign Magnitude Representation")]
#[weblab(weight=5)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::cmp::Ordering;
        use std::fmt::{Display, Formatter};
        use std::hash::Hash;
        use std::ops::Add;
        use weblab::solution_only;

        solution_only! {
            #[derive(Clone, Copy, Default, Debug)]
        }
        pub struct SignMagnitude(pub u32);

        impl SignMagnitude {
            pub fn sign(self) -> bool {
                (self.0 & 0x8000_0000) != 0
            }

            pub fn magnitude(self) -> u32 {
                self.0 & 0x7fff_ffff
            }

            solution_only! {
                fn normalize(self) -> SignMagnitude {
                    if self.sign() && self.magnitude() == 0 {
                        SignMagnitude(0)
                    } else {
                        self
                    }
                }

                fn to_i32(self) -> i32 {
                    if self.sign() {
                        ((!self.magnitude()).wrapping_add(1)) as i32
                    } else {
                        self.0 as i32
                    }
                }

                fn from_i32(from: i32) -> Self {
                    if from < 0 {
                        Self((!(from.wrapping_sub(1)) as u32) | 0x8000_0000)
                    } else {
                        Self(from as u32)
                    }
                }
            }
        }

        solution_only! {
            impl PartialEq for SignMagnitude {
                fn eq(&self, other: &Self) -> bool {
                    self.normalize().0 == other.normalize().0
                }
            }
            impl Eq for SignMagnitude {}

            impl PartialOrd<Self> for SignMagnitude {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            impl Ord for SignMagnitude {
                fn cmp(&self, other: &Self) -> Ordering {
                    if self.eq(other) { return Ordering::Equal }
                    match (self.sign(), other.sign()) {
                        (true, false) => Ordering::Less,
                        (false, true) => Ordering::Greater,
                        (false, false) => self.0.cmp(&other.0),
                        (true, true) => self.0.cmp(&other.0).reverse(),
                    }
                }
            }

            impl Add for SignMagnitude {
                type Output = SignMagnitude;

                fn add(self, rhs: Self) -> Self::Output {
                    Self::from_i32(self.to_i32() + rhs.to_i32())
                }
            }

            impl Display for SignMagnitude {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{}", if self.sign() {"-"} else {""}, self.magnitude())
                }
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use std::cmp::Ordering;
        use std::collections::HashSet;
        use std::hash::Hash;
        use weblab::{solution_only, template_only};

        solution_only! {
            macro_rules! test_each {
                ($($name:ident : $stmt: stmt);* $(;)?) => {
                    $(
                        #[test]
                        fn $name() {
                            $stmt
                        }
                    )*
                };
            }

            test_each! {
                eq1: assert_eq!(SignMagnitude(0), SignMagnitude(0));
                eq2: assert_eq!(SignMagnitude(0x80000000), SignMagnitude(0));
                eq3: assert_eq!(SignMagnitude(0), SignMagnitude(0x80000000));
                eq4: assert_eq!(SignMagnitude(3), SignMagnitude(3));
                eq5: assert_ne!(SignMagnitude(3), SignMagnitude(8));

                cmp1: assert!(SignMagnitude(0) < SignMagnitude(1));
                cmp2: assert!(SignMagnitude(15) < SignMagnitude(100));
                cmp3: assert!(SignMagnitude(15 | 0x80000000) < SignMagnitude(100));
                cmp4: assert!(SignMagnitude(15 | 0x80000000) > SignMagnitude(100 | 0x80000000));
                cmp5: assert!(SignMagnitude(15) < SignMagnitude(100));
                cmp6: assert!(SignMagnitude(0x80000000).cmp(&SignMagnitude(0)) == Ordering::Equal);

                add1: assert_eq!(SignMagnitude(15) + SignMagnitude(30), SignMagnitude(45));
                add2: assert_eq!(SignMagnitude(15 | 0x80000000) + SignMagnitude(30 | 0x80000000), SignMagnitude(45 | 0x80000000));
                add3: assert_eq!(SignMagnitude(15 | 0x80000000) + SignMagnitude(30), SignMagnitude(15));
                add4: assert_eq!(SignMagnitude(15) + SignMagnitude(30 | 0x80000000), SignMagnitude(15 | 0x80000000));
                add5: assert!(SignMagnitude(0) + SignMagnitude(0x80000000) == SignMagnitude(0) || SignMagnitude(0) + SignMagnitude(0x80000000) == SignMagnitude(0x80000000));

                display1: assert_eq!(SignMagnitude(15).to_string(), "15");
                display2: assert_eq!(SignMagnitude(15 | 0x80000000).to_string(), "-15");
                display3: assert_eq!(SignMagnitude(0).to_string(), "0");
                display4: assert!(SignMagnitude(0 | 0x80000000).to_string() == "0" || SignMagnitude(0 | 0x80000000).to_string() == "-0");

                default1: assert_eq!(SignMagnitude::default(), SignMagnitude(0));

                clone1: assert_eq!(SignMagnitude(1980).clone(), SignMagnitude(1980));
            }
        }
    }
}

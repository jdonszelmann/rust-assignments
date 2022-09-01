use weblab::weblab;

#[weblab(programming_assignment)]
/// In rust there are a grand total of *three* ways to compare
/// values. However, it's mostly worth  discussing two of them.
///
/// Normal equality is what the `==` operator is used for. When `a == b` returns
/// `true`, then `a` and `b` are equal.
///
/// However, there's also *structural equality*. Two values are structurally equal
/// if they follow the same structure. To use this type of equality, you can give
/// rust a pattern, and ask if a certain object follows the same structure as that
/// pattern. Let's look at an example:
///
/// ```rust
/// enum A {
///     X,
///     Y(i64),
/// }
///
/// fn main(v: A) {
///     match v {
///         // `A::X` is a pattern here saying "is v like an X?"
///         A::X => println!("we've got an x"),
///         // `A::Y(1)` is a pattern saying "is v like a Y with a 1 in it?"
///         A::Y(1) => println!("we've got a Y with a 1 in it")
///         // `A::Y(a)` is a pattern saying "is v like a Y, but with a
///         // number `a` in it, but only if a is larger than 3.
///         A::Y(a) if a > 3 => {
///             println("we've got a Y with a value greater than 3 ({})", a),
///         }
///         // the only other option is that v is a Y with either 1 or 2 in it:
///         _ => println!("v is a Y with either 1 or 2 in it")
///     }
/// }
/// ```
///
/// The match statement tries every pattern from top to bottom. The first pattern
/// that matches the value is chosen and executed.
///
/// In the exercise, you are given an enum and a number of patterns.
/// For each of the patterns, create an expression that would be matched with
/// the pattern. Note that there are often multiple correct answers.
#[weblab(title = "Structural Matching")]
#[weblab(weight = 2)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        pub enum ExerciseEnum {
            A(u8, i64),
            B([u8; 4]),
            /// you can ignore the 'static for now.
            /// There's a good reason for it but we will talk about it
            /// in future lectures. This just means there's a string
            /// stored in a C.
            C(&'static str),

            D {
                a: char,
                b: Option<i64>,
            },

            E(Result<i32, i32>),
        }

        /// `ExerciseEnum::A(1 | 2, 3..=10)`
        pub fn pat_1() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::A(1, 4)
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::A(_, b) if b > 3`
        pub fn pat_2() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::A(1, 5)
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::B([1, a, 2, b]) if b == a`
        pub fn pat_3() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::B([1, 2, 2, 2])
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::C("software" | "fundamentals")`
        pub fn pat_4() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::C("software")
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::D{a: 'a'..='f', b: None}`
        pub fn pat_5() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::D {a: 'd', b: None}
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::D{a: 'a', b: Some(x)} if x > 3`
        pub fn pat_6() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::D {a: 'a', b: Some(5)}
            }
            template_only! {
                todo!()
            }
        }

        /// `ExerciseEnum::E(Err(1..=5))`
        pub fn pat_7() -> ExerciseEnum {
            solution_only! {
                ExerciseEnum::E(Err(3))
            }
            template_only! {
                todo!()
            }
        }
    }

    #[weblab(test_template)]
    mod test_template {}

    #[weblab(test)]
    mod test {
        use super::solution::*;

        #[test]
        fn test_pat_1() {
            assert!(matches!(pat_1(), ExerciseEnum::A(1 | 2, 3..=10)))
        }

        #[test]
        fn test_pat_2() {
            assert!(matches!(pat_2(), ExerciseEnum::A(_, b) if b > 3))
        }

        #[test]
        fn test_pat_3() {
            assert!(matches!(pat_3(), ExerciseEnum::B([1, a, 2, b]) if b == a))
        }

        #[test]
        fn test_pat_4() {
            assert!(matches!(
                pat_4(),
                ExerciseEnum::C("software" | "fundamentals")
            ))
        }

        #[test]
        fn test_pat_5() {
            assert!(matches!(
                pat_5(),
                ExerciseEnum::D {
                    a: 'a'..='f',
                    b: None
                }
            ))
        }

        #[test]
        fn test_pat_6() {
            assert!(matches!(pat_6(), ExerciseEnum::D{a: 'a', b: Some(x)} if x > 3))
        }
        #[test]
        fn test_pat_7() {
            assert!(matches!(pat_7(), ExerciseEnum::E(Err(1..=5))))
        }
    }
}

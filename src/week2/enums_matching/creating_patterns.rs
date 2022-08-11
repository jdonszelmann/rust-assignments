use weblab::weblab;

#[weblab(programming_assignment)]
/// In the previous exercise we discussed pattern matching.
/// There, we asked you to create expressions which would be matched
/// by a pattern.
///
/// Here we will swap positions. You will need to make your own patterns
/// to fulfill the assignment
///
/// Use pattern matching for each assignment.
#[weblab(title = "Structural Matching")]
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

        /// Match only ExerciseEnum::A if the first number is greater than 3 and no other variants.
        pub fn pat_1(v: ExerciseEnum) -> bool {
            solution_only! {
                match v {
                    ExerciseEnum::A(a, _) if a > 3 => true,
                    _ => false,
                }
            }
            template_only! {
                todo!()
            }
        }

        /// Match both ExerciseEnum::A and ExerciseEnum::B given that
        /// the first number in them is even
        pub fn pat_2(v: ExerciseEnum) -> bool {
            solution_only! {
                match v {
                    ExerciseEnum::A(a, _) | ExerciseEnum::B([a, ..]) if a % 2 == 0 => true ,
                    _ => false,
                }
            }
            template_only! {
                todo!()
            }
        }

        pub fn pat_3(v: ExerciseEnum) -> bool {
            solution_only! {
                match v {

                    _ => false,
                }
            }
        }
    }

    #[weblab(test_template)]
    mod test_template {}

    #[weblab(test)]
    mod test {
        use std::mem;
        use super::solution::*;

        fn test_all_except(f: fn(ExerciseEnum) -> bool, except: &[ExerciseEnum]) {
            'outer: for i in [
                ExerciseEnum::A(0, 0),
                ExerciseEnum::B([0, 0, 0, 0]),
                ExerciseEnum::C("test"),
                ExerciseEnum::D { a: 'x', b: None },
                ExerciseEnum::E(Ok(3)),
            ] {
                for j in except {
                    if mem::discriminant(&i) == mem::discriminant(j) {
                        continue 'outer
                    }
                }
                assert!(!f(i));
            }
        }

        #[test]
        fn test_pat_1() {
            test_all_except(pat_1, &[ExerciseEnum::A(0, 0)]);
            assert!(!pat_1(ExerciseEnum::A(0, 0)));
            assert!(!pat_1(ExerciseEnum::A(1, 0)));
            assert!(!pat_1(ExerciseEnum::A(2, 0)));
            assert!(!pat_1(ExerciseEnum::A(3, 0)));
            assert!(pat_1(ExerciseEnum::A(4, 0)));
            assert!(!pat_1(ExerciseEnum::A(0, 4)));
        }

        #[test]
        fn test_pat_2() {
            test_all_except(pat_2, &[ExerciseEnum::A(0, 0), ExerciseEnum::B([0, 0, 0, 0])]);
            assert!(pat_2(ExerciseEnum::A(0, 0)));
            assert!(!pat_2(ExerciseEnum::A(1, 0)));
            assert!(pat_2(ExerciseEnum::A(2, 0)));
            assert!(!pat_2(ExerciseEnum::A(3, 0)));
            assert!(pat_2(ExerciseEnum::A(2, 1)));
            assert!(pat_2(ExerciseEnum::B([0, 0, 0, 0])));
            assert!(!pat_2(ExerciseEnum::B([1, 0, 0, 0])));
            assert!(pat_2(ExerciseEnum::B([2, 0, 0, 0])));
            assert!(!pat_2(ExerciseEnum::B([3, 2, 0, 0])));
        }
    }
}

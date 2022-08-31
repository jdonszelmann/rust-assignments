use weblab::weblab;

#[weblab(programming_assignment)]
/// In the previous exercise we discussed pattern matching.
/// There, we asked you to create expressions which would be matched
/// by a pattern.
///
/// Here we will swap positions. You will need to make your own patterns
/// based on the given conditions.
///
/// Use pattern matching for each assignment.
#[weblab(title = "Creating Patterns")]
#[weblab(weight = 3)]
mod assignment {
    #[weblab(solution)]
    #[allow(clippy::match_like_matches_macro)]
    mod solution {
        use weblab::{solution_only, template_only};

        pub enum ExerciseEnum {
            A(u8, i64),
            B([u8; 4]),
            /// You can ignore the 'static for now.
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
        /// the first number in them is even.
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

        /// v is a tuple of two enum values.
        /// return true, if both are a C with the same string in them,
        /// *or* if one of the two is a D, and the other a C and the first character in the string in C is
        /// equal to the character in D.
        ///
        /// So  pat_3((ExerciseEnum::C("test"), ExerciseEnum::D {a: 't', b: None})) should be true,
        /// and pat_3((ExerciseEnum::C("test"), ExerciseEnum::D {a: 'x', b: None})) should be false.
        pub fn pat_3(v: (ExerciseEnum, ExerciseEnum)) -> bool {
            solution_only! {
                match v {
                    (ExerciseEnum::C(a), ExerciseEnum::C(b)) if a == b => true,
                    (ExerciseEnum::C(s), ExerciseEnum::D {a, ..}) |
                    (ExerciseEnum::D {a, ..}, ExerciseEnum::C(s)) if s.starts_with(a) => true,
                    _ => false,
                }
            }
            template_only! {
                todo!()
            }
        }

        /// If `v` is an ExerciseEnum::E, it can either be Ok or Err.
        ///
        /// If it's an Err, return the Err as it is.
        /// If it's an Ok, return the Ok, but multiply the value by 2.
        ///
        /// Otherwise, return None.
        pub fn pat_4(v: ExerciseEnum) -> Option<ExerciseEnum> {
            solution_only! {
                match v {
                    ExerciseEnum::E(Ok(i)) => Some(ExerciseEnum::E(Ok(i * 2))),
                    ExerciseEnum::E(Err(e)) => Some(ExerciseEnum::E(Err(e))),
                    _ => None
                }
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
        use std::mem;

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
                        continue 'outer;
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
            test_all_except(
                pat_2,
                &[ExerciseEnum::A(0, 0), ExerciseEnum::B([0, 0, 0, 0])],
            );
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

        #[test]
        fn test_pat_3() {
            assert!(!pat_3((ExerciseEnum::A(0, 0), ExerciseEnum::A(0, 0))));
            assert!(!pat_3((ExerciseEnum::C("a"), ExerciseEnum::C("b"))));
            assert!(pat_3((ExerciseEnum::C("a"), ExerciseEnum::C("a"))));
            assert!(pat_3((
                ExerciseEnum::C("hottentottententententoonstelling"),
                ExerciseEnum::C("hottentottententententoonstelling")
            )));
        }

        #[test]
        fn test_pat_3_c_and_d() {
            assert!(pat_3((
                ExerciseEnum::C("test"),
                ExerciseEnum::D { a: 't', b: None }
            )));
            assert!(pat_3((
                ExerciseEnum::D { a: 't', b: None },
                ExerciseEnum::C("test")
            )));
            assert!(!pat_3((
                ExerciseEnum::C("test"),
                ExerciseEnum::D { a: 'x', b: None }
            )));
            assert!(!pat_3((
                ExerciseEnum::D { a: 'x', b: None },
                ExerciseEnum::C("test")
            )));
            assert!(!pat_3((
                ExerciseEnum::D { a: 'x', b: None },
                ExerciseEnum::D { a: 'x', b: None }
            )));
        }

        #[test]
        fn test_pat_4() {
            assert!(matches!(
                pat_4(ExerciseEnum::E(Ok(5))),
                Some(ExerciseEnum::E(Ok(10)))
            ));
            assert!(matches!(
                pat_4(ExerciseEnum::E(Ok(-5))),
                Some(ExerciseEnum::E(Ok(-10)))
            ));
            assert!(matches!(
                pat_4(ExerciseEnum::E(Err(5))),
                Some(ExerciseEnum::E(Err(5)))
            ));
            assert!(matches!(pat_4(ExerciseEnum::A(0, 0)), None));
        }
    }
}

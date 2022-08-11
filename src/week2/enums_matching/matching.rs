use weblab::weblab;

#[weblab(programming_assignment)]
/// In C, there is a "switch" block. With it you can write multiple conditions, and see if a variable satisfies one of the conditions.
///
/// In rust, a "match" block works like a more powerful "switch" block.
///
/// In each of the exercises, you are asked to return a value based on the input. Use "match" to solve these assignments
#[weblab(title = "Matching")]
mod assignment {
    #[weblab(solution)]
    #[allow(clippy::match_like_matches_macro)]
    mod solution {
        use weblab::{solution_only, template_only};

        // return 1 if v is either 1, 3 or 5
        // return 2 if v is either 0, 2 or 4
        // return 3 in any other situation
        pub fn exercise_1(v: u8) -> u8 {
            solution_only! {
                match v {
                    1 | 3 | 5 => 1,
                    0 | 2 | 4 => 2,
                    _ => 3,
                }
            }
            template_only! {
                todo!()
            }
        }

        // return 1 if v is between 0 (inclusive) and 100 (exclusive)
        // return 2 if v is between 100 (inclusive) and 200 (exclusive)
        // return 3 in any other situation
        pub fn exercise_2(v: u8) -> u8 {
            solution_only! {
                match v {
                    0..=99 => 1,
                    100..=199 => 2,
                    _ => 3,
                }
            }
            template_only! {
                todo!()
            }
        }

        // return 1 if v is in the first half of the latin/english alphabet (a-n) (uppercase *or* lowercase)
        // return 2 if v is in the second half of the latin/english alphabet (o-z) (uppercase *or* lowercase)
        // return 3 if v is not in the latin/english alphabet
        pub fn exercise_3(v: char) -> u8 {
            solution_only! {
                match v {
                    'a'..='n' | 'A'..='N' => 1,
                    'o'..='z' | 'O'..='Z' => 2,
                    _ => 3
                }
            }
            template_only! {
                todo!()
            }
        }

        // Check if the character in v is a digit (0-9)
        pub fn exercise_4(v: char) -> bool {
            solution_only! {
                match v {
                    '0'..='9' => true,
                    _ => false
                }
            }
            template_only! {
                todo!()
            }
        }

        // Check if v is a digit. If so, return a u8 with which number it is. Use a match block
        // and some math instead of one of the built-in functions. Hint: it may help to cast a char
        // to a number (`v as u32`) to solve this problem. In the standard library, there is
        // `char::to_digit() to solve this exact problem. Try not to use its implementation, but if you're
        // stuck, use it for inspiration and maybe to check your solution.
        //
        // If v is not a digit, return None.
        pub fn exercise_5(v: char) -> Option<u32> {
            solution_only! {
                match v {
                    '0'..='9' => Some(v as u32 - '0' as u32),
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

        #[test]
        fn test_ex_1() {
            assert_eq!(exercise_1(1), 1);
            assert_eq!(exercise_1(3), 1);
            assert_eq!(exercise_1(5), 1);
            assert_eq!(exercise_1(0), 2);
            assert_eq!(exercise_1(2), 2);
            assert_eq!(exercise_1(4), 2);

            for i in 0..=255u8 {
                if i > 5 {
                    assert_eq!(exercise_1(i), 3);
                }
            }
        }

        #[test]
        fn test_ex_2() {
            for i in 0..100u8 {
                assert_eq!(exercise_2(i), 1);
            }
            for i in 100..200u8 {
                assert_eq!(exercise_2(i), 2);
            }
            for i in 200..=255 {
                assert_eq!(exercise_2(i), 3);
            }
        }

        #[test]
        fn test_ex_3() {
            for i in 'a'..='n' {
                assert_eq!(exercise_3(i), 1);
            }
            for i in 'o'..='z' {
                println!("{}", i);
                assert_eq!(exercise_3(i), 2);
            }
            for i in 'A'..='N' {
                assert_eq!(exercise_3(i), 1);
            }
            for i in 'O'..='Z' {
                assert_eq!(exercise_3(i), 2);
            }
            for i in '0'..='9' {
                assert_eq!(exercise_3(i), 3);
            }
        }

        #[test]
        fn test_ex_4() {
            for i in '0'..='9' {
                assert!(exercise_4(i));
            }
            for i in 'A'..='z' {
                assert!(!exercise_4(i));
            }
        }

        #[test]
        fn test_ex_5() {
            for i in '0'..='z' {
                assert_eq!(exercise_5(i), i.to_digit(10));
            }
        }
    }
}

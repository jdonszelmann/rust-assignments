use weblab::weblab;

#[weblab(programming_assignment)]
/// Fizz buzz is a group word game for children to teach them about division.
/// Players take turns to count incrementally, replacing any number divisible by 3 with the word "Fizz", and any number divisible by 5 with the word "Buzz".
/// Any number that is divisible by 15 is replaced by "FizzBuzz".
///
/// In this exercise, you have to implement the FizzBuzzWoofYum game.
///
/// - Any number that is divisible by 3, is replaced by Fizz.
/// - Any number that is divisible by 5, is replaced by Buzz.
/// - Any number that is divisible by 7, is replaced by Woof.
/// - Any number that is divisible by 11, is replaced by Yum.
///
/// The first 30 numbers of the FizzBuzzWoofYum sequence are:
///
/// 1.  "1"
/// 2.  "2"
/// 3.  "Fizz"
/// 4.  "4"
/// 5.  "Buzz"
/// 6.  "Fizz"
/// 7.  "Woof"
/// 8.  "8"
/// 9.  "Fizz"
/// 10. "Buzz"
/// 11. "Yum"
/// 12. "Fizz"
/// 13. "13"
/// 14. "Woof"
/// 15. "FizzBuzz"
/// 16. "16"
/// 17. "17"
/// 18. "Fizz"
/// 19. "19"
/// 20. "Buzz"
/// 21. "FizzWoof"
/// 22. "Yum"
/// 23. "23"
/// 24. "Fizz"
/// 25. "Buzz"
/// 26. "26"
/// 27. "Fizz"
/// 28. "Woof"
/// 29. "29"
/// 30. "FizzBuzz"
///
/// Given a number n, output the first n numbers of the FizzBuzzWoofYum sequence.
/// The output should be a `Vec` of `String`s, each `String` being a separate number of the sequence.
///
/// Hint: There is a better way of doing this then writing 16 separate if statements.
#[weblab(title = "Fizz Buzz Woof Yum")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn fizzbuzzwoofyum(n: u64) -> Vec<String> {
            let mut buffer = vec![];
            for i in 1..=n {
                let mut subbuf = String::new();
                if i % 3 == 0 {
                    subbuf += "Fizz";
                }
                if i % 5 == 0 {
                    subbuf += "Buzz";
                }
                if i % 7 == 0 {
                    subbuf += "Woof";
                }
                if i % 11 == 0 {
                    subbuf += "Yum";
                }
                if subbuf.is_empty() {
                    subbuf += &i.to_string();
                }
                buffer.push(subbuf);
            }
            buffer
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn fizzbuzzwoofyum(n: u64) -> Vec<String> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_thirty() {
            assert_eq!(
                fizzbuzzwoofyum(30),
                vec![
                    "1", "2", "Fizz", "4", "Buzz", "Fizz", "Woof", "8", "Fizz", "Buzz", "Yum",
                    "Fizz", "13", "Woof", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "FizzWoof",
                    "Yum", "23", "Fizz", "Buzz", "26", "Fizz", "Woof", "29", "FizzBuzz",
                ]
            );
        }

        solution_only! {
            pub fn fizzbuzzwoofyum_ref(n: u64) -> Vec<String> {
                let mut buffer = vec![];
                for i in 1..=n {
                    let mut subbuf = String::new();
                    if i % 3 == 0 {
                        subbuf += "Fizz";
                    }
                    if i % 5 == 0 {
                        subbuf += "Buzz";
                    }
                    if i % 7 == 0 {
                        subbuf += "Woof";
                    }
                    if i % 11 == 0 {
                        subbuf += "Yum";
                    }
                    if subbuf.is_empty() {
                        subbuf += &i.to_string();
                    }
                    buffer.push(subbuf);
                }
                buffer
            }

            #[test]
            fn test_0() {
                assert_eq!(fizzbuzzwoofyum(0), fizzbuzzwoofyum_ref(0));
            }

            #[test]
            fn test_1() {
                assert_eq!(fizzbuzzwoofyum(1), fizzbuzzwoofyum_ref(1));
            }

             #[test]
            fn test_8() {
                assert_eq!(fizzbuzzwoofyum(8), fizzbuzzwoofyum_ref(8));
            }

            #[test]
            fn test_158() {
                assert_eq!(fizzbuzzwoofyum(158), fizzbuzzwoofyum_ref(158));
            }

            #[test]
            fn test_12098() {
                assert_eq!(fizzbuzzwoofyum(12098), fizzbuzzwoofyum_ref(12098));
            }
        }
    }
}

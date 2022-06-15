use weblab::weblab;

#[weblab(programming_assignment)]
/// Fizz buzz is a group word game for children to teach them about division.
/// Players take turns to count incrementally, replacing any number divisible by 3 with the word "Fizz", and any number divisible by 5 with the word "Buzz".
/// Any number that is divisible by 15 is replaced by "FizzBuzz".
///
/// The first 20 numbers of the FizzBuzz sequence are:
///
/// 1.  "1"
/// 2.  "2"
/// 3.  "Fizz"
/// 4.  "4"
/// 5.  "Buzz"
/// 6.  "Fizz"
/// 7.  "7"
/// 8.  "8"
/// 9.  "Fizz"
/// 10. "Buzz"
/// 11. "11"
/// 12. "Fizz"
/// 13. "13"
/// 14. "14"
/// 15. "FizzBuzz"
/// 16. "16"
/// 17. "17"
/// 18. "Fizz"
/// 19. "19"
/// 20. "Buzz"
///
/// Given a number n, output the first n numbers of the FizzBuzz sequence.
/// The output should be a `Vec` of `String`s, each `String` being a seperate number of the sequence.
#[weblab(title = "Fizz Buzz")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn fizzbuzz(n: u64) -> Vec<String> {
            let mut buffer = vec![];
            for i in 1..=n {
                let mut subbuf = String::new();
                if i % 3 == 0 {
                    subbuf += "Fizz";
                }
                if i % 5 == 0 {
                    subbuf += "Buzz";
                }
                if subbuf.len() == 0 {
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
        pub fn fizzbuzz(n: u64) -> Vec<String> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_twenty() {
            assert_eq!(fizzbuzz(20), vec![
                "1",
                "2",
                "Fizz",
                "4",
                "Buzz",
                "Fizz",
                "7",
                "8",
                "Fizz",
                "Buzz",
                "11",
                "Fizz",
                "13",
                "14",
                "FizzBuzz",
                "16",
                "17",
                "Fizz",
                "19",
                "Buzz",
            ]);
        }

        solution_only! {
            pub fn fizzbuzz_ref(n: u64) -> Vec<String> {
                let mut buffer = vec![];
                for i in 1..=n {
                    let mut subbuf = String::new();
                    if i % 3 == 0 {
                        subbuf += "Fizz";
                    }
                    if i % 5 == 0 {
                        subbuf += "Buzz";
                    }
                    if subbuf.len() == 0 {
                        subbuf += &i.to_string();
                    }
                    buffer.push(subbuf);
                }
                buffer
            }

            #[test]
            fn test_0() {
                assert_eq!(fizzbuzz(0), fizzbuzz_ref(0));
            }

            #[test]
            fn test_1() {
                assert_eq!(fizzbuzz(1), fizzbuzz_ref(1));
            }

             #[test]
            fn test_8() {
                assert_eq!(fizzbuzz(8), fizzbuzz_ref(8));
            }

            #[test]
            fn test_158() {
                assert_eq!(fizzbuzz(158), fizzbuzz_ref(158));
            }

            #[test]
            fn test_12098() {
                assert_eq!(fizzbuzz(12098), fizzbuzz_ref(12098));
            }
        }
    }
}

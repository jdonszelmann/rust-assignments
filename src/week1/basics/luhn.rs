use weblab::weblab;

#[weblab(programming_assignment)]
/// The Luhn algorithm (Wikipedia) is used to check bank card numbers for simple errors such as mistyping a digit, and proceeds as follows:
///
/// * consider each digit as a separate number;
/// * moving left, double every other number from the second last;
/// * subtract 9 from each number that is now greater than 9;
/// * add all the resulting numbers together;
/// * if the total is divisible by 10, the card number is valid.
///
/// Define a function `fn luhn_double(i64) -> i64;` that doubles a digit and subtracts 9 if the result is greater than 9. For example:
///
/// ```
/// > luhn_double(3)
/// 6
///
/// > luhn_double(6)
/// 3
/// ```
///
/// Using luhnDouble and the integer remainder function mod, define a function `luhn(a: i64, b: i64, c: i64, d: i64) -> bool;` that decides if a four-digit bank card number is valid. For example:
///
/// ```
/// > luhn(1, 7, 8, 4)
/// True
///
/// > luhn(4, 7, 8, 3)
/// False
/// ```
///
/// Now define a function `luhn_final(i64, i64, i64) -> bool;` that returns the fourth digit of a four-digit bank card number. For example:
///
/// ```
/// > luhn_final(1, 7, 8)
/// 4
///
/// > luhn_final(4, 7, 8)
/// 8
/// ```
///
/// NOTE: in `luhn_final`, do note that the modulus operator (`%`) works differently on negative numbers than you might expect.
/// Unfortunatly, the modulus operator is very hard to define properly.
/// Search for `rem_euclid`, which has more useful mathematical properties, especially on negative numbers.
#[weblab(title = "Luhn Algorithm")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        pub fn luhn_double(x: i64) -> i64 {
            solution_only! {
                let a = x * 2;
                if a > 9 {
                    a - 9
                } else {
                    a
                }
            }

            template_only! {todo!()}
        }

        pub fn luhn(a: i64, b: i64, c: i64, d: i64) -> bool {
            solution_only! {
                let total = luhn_double(a) + b + luhn_double(c) + d;
                total.rem_euclid(10) == 0
            }

            template_only! {todo!()}
        }

        pub fn luhn_final(a: i64, b: i64, c: i64) -> i64 {
            solution_only! {
                let total = 10 - (luhn_double(a) + b + luhn_double(c));
                total.rem_euclid(10)
            }

            template_only! {todo!()}
        }
    }

    #[weblab(test)]
    mod test {
        use weblab::solution_only;
        /// These imports may be unused if you don't have tests. The `allow` line
        /// simply makes it so the compiler won't give a warning about this.
        #[allow(unused_imports)]
        use super::solution::{luhn_double, luhn, luhn_final};

        #[test]
        fn test_luhn_double() {
            assert_eq!(luhn_double(3), 6);
            assert_eq!(luhn_double(6), 3);
        }

        #[test]
        fn test_luhn() {
            assert!(luhn(1, 7, 8, 4));
            assert!(!luhn(4, 7, 8, 3));
        }

        #[test]
        fn test_luhn_final() {
            assert_eq!(luhn_final(1, 7, 8), 4);
            assert_eq!(luhn_final(4, 7, 8), 8);
        }

        solution_only! {
            use quickcheck::quickcheck;

            fn luhn_spec(a: i64, b: i64, c: i64, d: i64) -> bool {
                let total = luhn_double_spec(a) + b + luhn_double_spec(c) + d;
                total.rem_euclid(10) == 0
            }

            fn luhn_double_spec(x: i64) -> i64 {
                let a = x * 2;
                if a > 9 {
                    a - 9
                } else {
                    a
                }
            }

            quickcheck! {
                fn prop_luhn_double_correct(x: u32) -> bool {
                    luhn_double(x as i64) == luhn_double_spec(x as i64)
                }
            }

            quickcheck! {
                fn prop_luhn_correct(a: u32, b: u32, c: u32, d: u32) -> bool {
                    luhn(a as i64, b as i64, c as i64, d as i64) == luhn_spec(a as i64, b as i64, c as i64, d as i64)
                }
            }

            quickcheck! {
                fn prop_luhn_final_correct(a: u32, b: u32, c: u32) -> bool {
                    luhn_spec(a as i64, b as i64, c as i64, luhn_final(a as i64, b as i64, c as i64))
                }
            }
        }
    }
}

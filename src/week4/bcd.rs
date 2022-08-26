use weblab::weblab;

#[weblab(programming_assignment)]
/// Binary Coded Decimal is way of representing numbers. Each decimal digit (0-9) is represented using a 4-bit number.
/// The digits have the following representations:
/// $$
/// 0: 0000
/// $$
/// $$
/// 1: 0001
/// $$
/// $$
/// 2: 0010
/// $$
/// $$
/// 3: 0011
/// $$
/// $$
/// 4: 0100
/// $$
/// $$
/// 5: 0101
/// $$
/// $$
/// 6: 0110
/// $$
/// $$
/// 7: 0111
/// $$
/// $$
/// 8: 1000
/// $$
/// $$
/// 9: 1001
/// $$
/// (note that 1100-1111 are unused)
///
/// A few examples of how numbers would be represented in BCD:
///
/// - 123 is represented as `0001 0010 0011`
/// - 890 is represented as `1000 1001 0000`
/// - 0 is represented as `0000`
///
/// Note that we can insert arbitrary zero digits before the BCD digits, to reach the desired number of BCD digits.
/// 123 could also be represented as `0000 0001 0010 0011` or `0000 0000 0000 0000 0000 0001 0010 0011`.
///
/// For this assignment, implement two functions:
///
/// 1. `from_bcd(u64) -> u64`, which converts 16 bcd digits to a "normal" binary u64 number.
/// 2. `to_bcd(u64) -> u64`, which converts a "normal" binary u64 number to 16 bcd digits.
///     For numbers that are out of range (such as 2^63, which can not be represented as 16-digit BCD) the function should panic.
#[weblab(title = "Binary Coded Decimal")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn from_bcd(mut bcd: u64) -> u64 {
            let mut result = 0u64;
            for _ in 0..16 {
                //Add 10 to the result, so we get a new digit at the lsb side
                result *= 10;
                // Get the most-significant bcd digit
                let nibble = bcd & (0b1111 << 60);
                // Add those 4 bits to the result
                result += nibble >> 60;
                // Shift bcd left 4, ready for next iteration
                bcd <<= 4;
            }
            result
        }

        pub fn to_bcd(mut num: u64) -> u64 {
            let mut bcd = 0;
            let mut i = 0;
            while num > 0 {
                //Take lowest decimal digit
                let nibble = num % 10;
                //And remove the digit
                num /= 10;
                //Add the digit to the bcd at the correct place
                bcd |= nibble << i;
                //Update correct place
                i += 4;
            }
            bcd
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn from_bcd(bcd: u64) -> u64 {
            todo!()
        }

        pub fn to_bcd(num: u64) -> u64 {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn from_bcd_example() {
            assert_eq!(from_bcd(0b0001_0010_0011), 123);
            assert_eq!(from_bcd(0b1000_1001_0000), 890);
            assert_eq!(from_bcd(0b0), 0);
        }

        #[test]
        fn to_bcd_example() {
            assert_eq!(0b0001_0010_0011, to_bcd(123));
            assert_eq!(0b1000_1001_0000, to_bcd(890));
            assert_eq!(0b0, to_bcd(0));
        }

        solution_only! {
            fn to_bcd_ref(mut num: u64) -> u64 {
                let mut bcd = 0;
                let mut i = 0;
                while num > 0 {
                    //Take lowest decimal digit
                    let nibble = num % 10;
                    //And remove the digit
                    num /= 10;
                    //Add the digit to the bcd at the correct place
                    bcd |= nibble << i;
                    //Update correct place
                    i += 4;

                }
                bcd
            }

            #[test]
            fn from_bcd_full_test() {
                for num in 0..1000000 {
                    assert_eq!(from_bcd(to_bcd_ref(num)), num);
                }
            }

            #[test]
            fn to_bcd_full_test() {
                for num in 0..1000000 {
                    assert_eq!(to_bcd(num), to_bcd_ref(num));
                }
            }

            #[test]
            #[should_panic]
            fn to_bcd_panic_test() {
                to_bcd(10000000000000000);
            }
        }
    }
}

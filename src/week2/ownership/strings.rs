use weblab::weblab;

#[weblab(programming_assignment)]
/// In this assignment, you will learn how to interact with strings in Rust.
/// We will consider 2 ways of representing strings:
///
/// - `String`, the owned string type
/// - `str`, a string slice, usually seen in its borrowed form `&str`
///
/// Implement all of the functions.
#[weblab(title = "Hello Words")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        /// Return a `String` containing the content "Hello World"
        pub fn make_a_string() -> String {
            String::from("Hello World")
        }

        /// Given two `&str`s, concatenate them
        /// For example: concat("abc", "def") == "abcdef"
        pub fn concat(a: &str, b: &str) -> String {
            String::from(a) + b
        }

        /// Given two `&str`s, interleave them.
        /// The strings are guaranteed to be the same length.
        /// For example: concat("abc", "def") == "adbecf"
        pub fn interleave(a: &str, b: &str) -> String {
            let mut buffer = String::with_capacity(a.len() + b.len());

            // The solution without iterators
            let mut rest_a = a;
            let mut rest_b = b;
            while !rest_a.is_empty() && !rest_b.is_empty() {
                let ca = rest_a.chars().next().unwrap();
                let cb = rest_b.chars().next().unwrap();

                buffer.push(ca);
                buffer.push(cb);

                rest_a = &rest_a[ca.len_utf8()..];
                rest_b = &rest_b[cb.len_utf8()..];
            }

            // The solution with iterators
            // for (ca, cb) in a.chars().zip(b.chars()) {
            //     buffer.push(ca);
            //     buffer.push(cb);
            // }

            buffer
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// Return a `&str` containing the content "Hello World"
        pub fn make_a_str() -> &'static str {
            todo!()
        }

        /// Return a `String` containing the content "Hello World"
        pub fn make_a_string() -> String {
            todo!()
        }

        /// Given two `&str`s, concatenate them
        /// For example: concat("abc", "def") == "abcdef"
        pub fn concat(a: &str, b: &str) -> String {
            todo!()
        }

        /// Given two `&str`s, interleave them.
        /// The strings are guaranteed to be the same length.
        /// For example: concat("abc", "def") == "adbecf"
        pub fn interleave(a: &str, b: &str) -> String {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_simple() {
            assert_eq!(concat("abc", "def"), "abcdef");
            assert_eq!(interleave("abc", "def"), "adbecf");
        }

        solution_only! {
            #[test]
            fn test_helloworld() {
                assert_eq!(make_a_string(), "Hello World")
            }

            #[test]
            fn concat_empty() {
                assert_eq!(concat("", ""), "");
            }

            #[test]
            fn concat_few() {
                assert_eq!(concat("x", "y"), "xy");
                assert_eq!(concat("aa", "bb"), "aabb");
                assert_eq!(concat("xasdasasjasdjl", "aslasjkl"), "xasdasasjasdjlaslasjkl");
            }

            #[test]
            fn concat_very_uneven() {
                assert_eq!(concat("", "yasdjalsjlasdkjasdjkasdklashjkl"), "yasdjalsjlasdkjasdjkasdklashjkl");
                assert_eq!(concat("yasdjalsjlasdkjasdjkasdklashjkl", ""), "yasdjalsjlasdkjasdjkasdklashjkl");
            }

            #[test]
            fn interleave_empty() {
                assert_eq!(interleave("", ""), "");
            }

            #[test]
            fn interleave_few() {
                assert_eq!(interleave("x", "y"), "xy");
                assert_eq!(interleave("aa", "bb"), "abab");
                assert_eq!(interleave("abcdefgh", "abcdefgh"), "aabbccddeeffgghh");
            }

            #[test]
            fn interleave_more() {
                assert_eq!(interleave("[]()", "<>{}"), "[<]>({)}");
                assert_eq!(interleave("1029", "53u1"), "15032u91");
            }
        }
    }
}
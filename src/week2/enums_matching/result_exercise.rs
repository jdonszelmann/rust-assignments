use weblab::weblab;

#[weblab(programming_assignment)]
/// For this assignment, you will be building a function that parses an `Account` from a String. This string has the format `fullname;username;age`.
/// If the string does not satisfy the format, the `decode_account` function should return the corresponding error.
///
/// For example, the following are correct account strings:
///
/// * `Elton Hercules John;eltonjohn;75`
/// * `David Bowie;drjones;69`
/// * `Bob Dylan;robertd;81`
///
/// The account strings have to satisfy the following properties:
///
/// * The string must consist of 3 parts separated by `;`. The parts may not contain the `;` character. If this condition is not met, return the `InvalidFormat` error.
/// * The full name must be at least 3 characters long. If this is not the case, return the `InvalidFullName` error.
/// * The username must be at least 3 characters long and none of the characters may be uppercase. If this is not the case, return the `InvalidUserName` error.
/// * The age must be a number between 0 and 150 (inclusive). If this is not the case, return the `InvalidAge` error.
///
/// For the `InvalidFullName`, `InvalidUserName`, and `InvalidAge` errors, include the part of the string (so the full name, username and age respectively) as the enum variant.
/// If the string contains multiple error, return the first property that was violated.
#[weblab(title = "Account Decoding")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        #[derive(PartialEq, Eq, Debug)]
        pub enum AccountDecodeError {
            InvalidFormat,
            InvalidFullName(String),
            InvalidUserName(String),
            InvalidAge(String)
        }

        #[derive(PartialEq, Eq, Debug)]
        pub struct Account {
            pub full_name: String,
            pub user_name: String,
            pub age: u32,
        }

        pub fn decode_account(input: &str) -> Result<Account, AccountDecodeError> {
            template_only! {
                todo!()
            }
            solution_only! {
                let parts: Vec<&str> = input.split(";").collect();
                if parts.len() != 3 { return Err(AccountDecodeError::InvalidFormat) }

                let full_name: String = parts[0].into();
                if full_name.len() < 3 { return Err(AccountDecodeError::InvalidFullName(full_name)) }

                let user_name: String = parts[1].into();
                if user_name.len() < 3 || user_name.chars().any(|c| c.is_uppercase()) { return Err(AccountDecodeError::InvalidUserName(user_name)) }

                let age_str: String = parts[2].into();
                let age: u32 = age_str.parse().map_err(|_| AccountDecodeError::InvalidAge(age_str.clone()))?;
                if age > 150 { return Err(AccountDecodeError::InvalidAge(age_str)) }

                Ok(Account{full_name, user_name, age})
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        template_only! {
            #[test]
            fn test_examples() {
                assert_eq!(decode_account("Elton Hercules John;eltonjohn;75"), Ok(Account {
                    full_name: "Elton Hercules John".to_string(),
                    user_name: "eltonjohn".to_string(),
                    age: 75
                }));
            }
        }

        solution_only! {
            #[test]
            fn test_good_cases() {
                assert_eq!(decode_account("Elton Hercules John;eltonjohn;75"), Ok(Account {
                    full_name: "Elton Hercules John".to_string(),
                    user_name: "eltonjohn".to_string(),
                    age: 75
                }));
                assert_eq!(decode_account("David Bowie;drjones;69"), Ok(Account {
                    full_name: "David Bowie".to_string(),
                    user_name: "drjones".to_string(),
                    age: 69
                }));
                assert_eq!(decode_account("Bob Dylan;robertd;81"), Ok(Account {
                    full_name: "Bob Dylan".to_string(),
                    user_name: "robertd".to_string(),
                    age: 81
                }));
            }

            #[test]
            fn test_edge_cases() {
                assert_eq!(decode_account("Abc;abc;0"), Ok(Account {
                    full_name: "Abc".to_string(),
                    user_name: "abc".to_string(),
                    age: 0
                }));
                assert_eq!(decode_account("Abcdefghijklmnopqrstuvwxyz;abcdefghijklmnopqrstuvwxyz;150"), Ok(Account {
                    full_name: "Abcdefghijklmnopqrstuvwxyz".to_string(),
                    user_name: "abcdefghijklmnopqrstuvwxyz".to_string(),
                    age: 150
                }));
            }

            #[test]
            fn test_incorrect_format() {
                assert_eq!(decode_account("Elton Hercules John;eltonjohn;75;"), Err(AccountDecodeError::InvalidFormat));
                assert_eq!(decode_account(";Elton Hercules John;eltonjohn;75"), Err(AccountDecodeError::InvalidFormat));
                assert_eq!(decode_account("Elton Hercules John;eltonjohn;;75"), Err(AccountDecodeError::InvalidFormat));
                assert_eq!(decode_account("Elton Hercules John;elton;john;75;"), Err(AccountDecodeError::InvalidFormat));
                assert_eq!(decode_account("w;x;y;z"), Err(AccountDecodeError::InvalidFormat));
            }

            #[test]
            fn test_incorrect_fullname() {
                assert_eq!(decode_account(";eltonjohn;75"), Err(AccountDecodeError::InvalidFullName("".into())));
                assert_eq!(decode_account("a;eltonjohn;75"), Err(AccountDecodeError::InvalidFullName("a".into())));
                assert_eq!(decode_account("ab;eltonjohn;75"), Err(AccountDecodeError::InvalidFullName("ab".into())));
                assert_eq!(decode_account("87;eltonjohn;75"), Err(AccountDecodeError::InvalidFullName("87".into())));
            }

            #[test]
            fn test_incorrect_username() {
                assert_eq!(decode_account("Elton Hercules John;;75"), Err(AccountDecodeError::InvalidUserName("".into())));
                assert_eq!(decode_account("Elton Hercules John;Elly;75"), Err(AccountDecodeError::InvalidUserName("Elly".into())));
                assert_eq!(decode_account("Elton Hercules John;el;75"), Err(AccountDecodeError::InvalidUserName("el".into())));
                assert_eq!(decode_account("Elton Hercules John;87;75"), Err(AccountDecodeError::InvalidUserName("87".into())));
                assert_eq!(decode_account("Elton Hercules John;ELLY;75"), Err(AccountDecodeError::InvalidUserName("ELLY".into())));
            }

            #[test]
            fn test_incorrect_age() {
                assert_eq!(decode_account("Elton Hercules John;eltonjosh;-1"), Err(AccountDecodeError::InvalidAge("-1".into())));
                assert_eq!(decode_account("Elton Hercules John;eltonjosh;151"), Err(AccountDecodeError::InvalidAge("151".into())));
                assert_eq!(decode_account("Elton Hercules John;eltonjosh;age"), Err(AccountDecodeError::InvalidAge("age".into())));
                assert_eq!(decode_account("Elton Hercules John;eltonjosh;seventeen"), Err(AccountDecodeError::InvalidAge("seventeen".into())));
                assert_eq!(decode_account("Elton Hercules John;eltonjosh;"), Err(AccountDecodeError::InvalidAge("".into())));
            }

        }
    }
}

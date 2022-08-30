use weblab::weblab;

#[weblab(programming_assignment)]
/// Different programming languages have different standards for the formatting and capitalization of function names, where function names consist of multiple words.
///
/// Rust's style usually prefers "snake case" names, where all words in the function name are seperated by underscores, and no capital letters are used.
/// For example, `is_snake_case` and `to_camel_case` are valid function names.
/// The rules for snake case are:
///
/// 1. All characters in the words are `alphanumeric` (part of an alphabet or number system).
///     In this convention, the name `to_%` is not valid since it contains a `%` which is not alphanumeric.
///     To check this you can use `is_alphanumeric` function on characters.
/// 2. None of the characters are uppercase.
///     Some characters such as `3` are neither uppercase nor lowercase, these characters are allowed.
///     In this convention, the name `is_Snake_case` is not valid since it contains a `S` which is uppercase.
///     To check this you can use `is_uppercase` function on characters.
/// 3. Words cannot be empty, that is, a function name cannot contain two consecutive underscores. For example, `is__snake_case` is not valid under the convention.
/// 4. A function name cannot start or end with an underscore. For example, `_is_snake_case` is not valid under the convention.
///
/// Implement a function `is_snake_case` that checks if a string satisfies the rules for snake case. If the input satiesfies all rules, return None. Otherwise, return what rule is the **first** one that is broken.
///
/// ---
///
/// Java uses a different convention called "lower camel case", where all words are concatenated without underscores, and all words except the first start with a capital letter.
/// For example, `isSnakeCase` and `toCamelCase` are function names that follow the convention.
///
/// Given a name that is in snake case, convert it to camel case.
/// If the input is not valid snake case, return an error. Use the `is_snake_case` function you wrote to achieve this.
///
/// Examples:
///
/// - to_camel_case("is_snake_case") == Ok("isSnakeCase")
/// - to_camel_case("another_example") = Ok("anotherExample")
/// - to_camel_case("not_Legal") = Err(SnakeCaseError::Uppercase)
///
#[weblab(title = "Case Converter")]
#[weblab(weight=5)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        #[derive(Eq, PartialEq, Debug)]
        pub enum SnakeCaseError {
            /// Found a character that is not alphanumeric (rule 1)
            NotAlphanumeric,
            /// Found a character that is uppercase (rule 2)
            Uppercase,
            /// Found an underscore where it is not allowed (rule 3 or 4)
            IncorrectUnderscore,
        }

        pub fn is_snake_case(input: &str) -> Result<(), SnakeCaseError> {
            let mut allow_underscore = false;
            for char in input.chars() {
                match char {
                    '_' if allow_underscore => {
                        allow_underscore = false;
                    }
                    '_' if !allow_underscore => return Err(SnakeCaseError::IncorrectUnderscore),
                    c if !c.is_alphanumeric() => return Err(SnakeCaseError::NotAlphanumeric),
                    c if c.is_uppercase() => return Err(SnakeCaseError::Uppercase),
                    _ => {
                        allow_underscore = true;
                    }
                }
            }
            if allow_underscore {
                Ok(())
            } else {
                Err(SnakeCaseError::IncorrectUnderscore)
            }
        }

        pub fn to_camel_case(input: &str) -> Result<String, SnakeCaseError> {
            is_snake_case(input)?;

            let mut buffer = String::with_capacity(input.len());
            let mut next_uppercase = false;
            for c in input.chars() {
                match c {
                    '_' => {
                        next_uppercase = true;
                    }
                    c if next_uppercase => {
                        next_uppercase = false;
                        c.to_uppercase().for_each(|c| buffer.push(c));
                    }
                    c => buffer.push(c),
                }
            }

            Ok(buffer)
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// Do not change the definition of SnakeCaseError
        #[derive(Eq, PartialEq, Debug)]
        pub enum SnakeCaseError {
            /// Found a character that is not alphanumeric (rule 1)
            NotAlphanumeric,
            /// Found a character that is uppercase (rule 2)
            Uppercase,
            /// Found an underscore where it is not allowed (rule 3 or 4)
            IncorrectUnderscore,
        }

        pub fn is_snake_case(input: &str) -> Result<(), SnakeCaseError> {
            todo!()
        }

        pub fn to_camel_case(input: &str) -> Result<String, SnakeCaseError> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::solution_only;

        #[test]
        pub fn examples_is_snakecase() {
            assert_eq!(is_snake_case("is_snake_case"), Ok(()));
            assert_eq!(is_snake_case("to_camel_case"), Ok(()));
            assert_eq!(is_snake_case("is_3"), Ok(()));

            assert_eq!(is_snake_case("is_%"), Err(SnakeCaseError::NotAlphanumeric));
            assert_eq!(
                is_snake_case("is_Snake_case"),
                Err(SnakeCaseError::Uppercase)
            );
            assert_eq!(
                is_snake_case("is__snake_case"),
                Err(SnakeCaseError::IncorrectUnderscore)
            );
            assert_eq!(
                is_snake_case("_is_snake_case"),
                Err(SnakeCaseError::IncorrectUnderscore)
            );
        }

        #[test]
        pub fn examples_to_camel_case() {
            assert_eq!(
                to_camel_case("is_snake_case"),
                Ok("isSnakeCase".to_string())
            );
            assert_eq!(
                to_camel_case("another_example"),
                Ok("anotherExample".to_string())
            );
            assert_eq!(to_camel_case("not_Legal"), Err(SnakeCaseError::Uppercase));
        }

        solution_only! {
            macro_rules! test_each {
                ($($name:ident : $stmt: stmt);* $(;)?) => {
                    $(
                        #[test]
                        fn $name() {
                            $stmt
                        }
                    )*
                };
            }

            test_each! {
                issnake1: assert_eq!(is_snake_case("is_snake_case"), Ok(()));
                issnake2: assert_eq!(is_snake_case("is_snake"), Ok(()));
                issnake3: assert_eq!(is_snake_case("is"), Ok(()));
                issnake4: assert_eq!(is_snake_case("i"), Ok(()));
                issnake5: assert_eq!(is_snake_case("i2s_snak3e_c1ase"), Ok(()));
                issnake6: assert_eq!(is_snake_case("I"), Err(SnakeCaseError::Uppercase));
                issnake7: assert_eq!(is_snake_case("Is"), Err(SnakeCaseError::Uppercase));
                issnake8: assert_eq!(is_snake_case("Si"), Err(SnakeCaseError::Uppercase));
                issnake9: assert_eq!(is_snake_case("is_snake_case_"), Err(SnakeCaseError::IncorrectUnderscore));
                issnake10: assert_eq!(is_snake_case("is_snake_case__"), Err(SnakeCaseError::IncorrectUnderscore));
                issnake11: assert_eq!(is_snake_case("is_sna__ke_case"), Err(SnakeCaseError::IncorrectUnderscore));
                issnake12: assert_eq!(is_snake_case("&"), Err(SnakeCaseError::NotAlphanumeric));
                issnake13: assert_eq!(is_snake_case("hey*hey"), Err(SnakeCaseError::NotAlphanumeric));

                tocamel1: assert_eq!(to_camel_case("pizza"), Ok("pizza".to_string()));
                tocamel2: assert_eq!(to_camel_case("with_5_numbers"), Ok("with5Numbers".to_string()));
                tocamel3: assert_eq!(to_camel_case("with_a5_numbers"), Ok("withA5Numbers".to_string()));
                tocamel4: assert_eq!(to_camel_case("with_5a_numbers"), Ok("with5aNumbers".to_string()));
                tocamel5: assert_eq!(to_camel_case("with_5A_numbers"), Err(SnakeCaseError::Uppercase));
                tocamel6: assert_eq!(to_camel_case("a_b_c"), Ok("aBC".to_string()));
            }
        }
    }
}

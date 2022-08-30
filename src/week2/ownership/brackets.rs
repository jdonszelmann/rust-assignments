use weblab::weblab;

#[weblab(programming_assignment)]
/// A parenthesis-balanced string is a string where each parenthesis is closed in the correct order.
/// For example, `(())`, `([])`, `([][][])`, and `([])([])` are parenthesis-balanced.
/// `([)]`, `(` and `(]` are not parenthesis-balanced.
/// You may assume the string only consists of `(`, `)`, `[` and `]` characters.
///
/// Return whether a given storing is parenthesis-balanced.
#[weblab(title = "Brackets")]
#[weblab(weight = 5)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::collections::VecDeque;
        use weblab::{solution_only, template_only};

        pub fn is_parenthesis_balanced(str: &str) -> bool {
            solution_only! {
                let mut stack = Vec::new();
                for c in str.chars() {
                    match c {
                        '(' => stack.push(')'),
                        '[' => stack.push(']'),
                        ')' | ']' => {
                            if let Some(cs) = stack.pop() {
                                if cs != c { return false; }
                            } else {
                                return false;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                stack.is_empty()
            }
            template_only! {
                todo!()
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};
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
                t1: assert!(is_parenthesis_balanced("(())"));
                t2: assert!(is_parenthesis_balanced("([])"));
                t3: assert!(is_parenthesis_balanced("([][][])"));
                t4: assert!(is_parenthesis_balanced("([])([])"));
                t5: assert!(is_parenthesis_balanced("([])([()[]])"));
                t6: assert!(!is_parenthesis_balanced("([)]"));
                t7: assert!(!is_parenthesis_balanced("(]"));
                t8: assert!(!is_parenthesis_balanced("("));
                t9: assert!(!is_parenthesis_balanced("()("));
                t10: assert!(!is_parenthesis_balanced("([)"));
                t11: assert!(!is_parenthesis_balanced("[(])"));
                t12: assert!(!is_parenthesis_balanced("[(])([)]"));
                t13: assert!(is_parenthesis_balanced("[([]()[])]"));
            }
        }
    }
}

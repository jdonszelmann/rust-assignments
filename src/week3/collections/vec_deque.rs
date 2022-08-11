use weblab::weblab;

#[weblab(programming_assignment)]
/// A VecDeque is a Deque, which is a double-ended queue. It is effectively a Stack (which you implemented in week 2) and a queue in one.
/// This means elements can be inserted and removed from both sides of this list in O(1) (the time needed does not change with the size of the queue).
/// This is in contrast with a Vec, where inserting of removing an element from the start is O(n) (the time needed grows linearly with the size of the vec), since all elements in the underlying array need to be shifted by one.
///
/// A parenthesis-balanced string is a string where each parenthesis is closed in the correct order.
/// For example, `(())`, `([])`, `([][][])`, and `([])([])` are parenthesis-balanced.
/// `([)]`, `(` and `(]` are not parenthesis-balanced.
/// You may assume the string only consists of `(`, `)`, `[` and `]` characters.
///
/// Return whether a given string is parenthesis-balanced. You have to implement this efficiently, using a `VecDeque`.
#[weblab(title = "Balanced Parenthesis")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::collections::VecDeque;
        use weblab::{solution_only, template_only};

        pub fn is_parenthesis_balanced(str: &str) -> bool {
            solution_only! {
                let mut stack: VecDeque<char> = VecDeque::new();
                for c in str.chars() {
                    match c {
                        '(' => stack.push_back(')'),
                        '[' => stack.push_back(']'),
                        ')' | ']' => {
                            if let Some(cs) = stack.pop_back() {
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
            #[test]
            fn test_paranthesis_balanced() {
                assert!(is_parenthesis_balanced("(())"));
                assert!(is_parenthesis_balanced("([])"));
                assert!(is_parenthesis_balanced("([][][])"));
                assert!(is_parenthesis_balanced("([])([])"));
                assert!(is_parenthesis_balanced("([])([()[]])"));

                assert!(!is_parenthesis_balanced("([)]"));
                assert!(!is_parenthesis_balanced("(]"));
                assert!(!is_parenthesis_balanced("("));
                assert!(!is_parenthesis_balanced("()("));
                assert!(!is_parenthesis_balanced("([)"));
            }
        }
    }
}

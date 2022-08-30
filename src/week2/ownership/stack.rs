use weblab::weblab;

#[weblab(programming_assignment)]
/// In this assignment, you're asked to implement the stack data structure.
/// A stack is a collection of elements with the FILO property: First In, Last Out.
/// So the retrieved element is always the element that was most recently stored. The stack should support
/// the following operations:
///
/// - Push, which adds an element to the top of the stack.
/// - Pop, which removes an element from the top of the stack.
///   In case no elements remain, return 0.
/// - Len, which returns the amount of elements that are currently placed on the stack.
///
/// In case the data structure is not clear, you can find more information here:
/// https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
///
/// The stack methods need to have the following signatures:
/// ```
/// pub fn new() -> Stack
/// pub fn push(..., value: i64)
/// pub fn pop(...) -> i64
/// pub fn len(...) -> usize
/// ```
///
/// For each of the methods, you have to decide whether the method should take `self`, `&self` or `&mut self`.
///
/// Hint: Use a `Vec` to store the elements of the stack.
#[weblab(title = "Wacky Stacky")]
#[weblab(weight = 1)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub struct Stack {
            vec: Vec<i64>,
        }

        impl Stack {
            /// Creates a new Stack.
            pub fn new() -> Stack {
                Stack { vec: Vec::new() }
            }

            /// Adds an element to the top of the stack.
            pub fn push(&mut self, value: i64) {
                self.vec.push(value)
            }

            /// Removes an element from the top of the stack.
            /// In case the stack is empty, return 0.
            pub fn pop(&mut self) -> i64 {
                if self.vec.is_empty() {
                    0
                } else {
                    self.vec.remove(self.vec.len() - 1)
                }
                // Or self.vec.pop().unwrap_or(0)
            }

            /// Returns the amount of elements currently on the stack.
            pub fn len(&self) -> usize {
                self.vec.len()
            }
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        use weblab::template_only;

        /// TODO: Add attributes of your choice
        pub struct Stack {}

        // TODO: Implement the methods
        impl Stack {}
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_simple() {
            let mut stack = Stack::new();
            stack.push(5);
            assert_eq!(stack.len(), 1);
            assert_eq!(stack.pop(), 5);
            assert_eq!(stack.pop(), 0);
        }

        solution_only! {
            #[test]
            fn test_len_is_not_mut() {
                let stack = Stack::new();
                let stack_ref = &stack;
                assert_eq!(stack_ref.len(), 0);
            }

            #[test]
            fn test_empty() {
                let mut stack = Stack::new();
                assert_eq!(stack.len(), 0);
                assert_eq!(stack.pop(), 0);
                assert_eq!(stack.pop(), 0);
                assert_eq!(stack.len(), 0);
            }

            #[test]
            fn test_multi1() {
                let mut stack = Stack::new();
                stack.push(15);
                stack.push(30);
                stack.push(45);
                stack.push(60);
                stack.push(75);
                assert_eq!(stack.len(), 5);
                assert_eq!(stack.pop(), 75);
                assert_eq!(stack.pop(), 60);
                assert_eq!(stack.len(), 3);
                assert_eq!(stack.pop(), 45);
                assert_eq!(stack.pop(), 30);
                assert_eq!(stack.len(), 1);
                assert_eq!(stack.pop(), 15);
                assert_eq!(stack.len(), 0);
                assert_eq!(stack.pop(), 0);
                assert_eq!(stack.len(), 0);
                assert_eq!(stack.pop(), 0);
            }

            #[test]
            fn test_multi_mixed() {
                let mut stack = Stack::new();
                stack.push(15);
                stack.push(30);
                assert_eq!(stack.len(), 2);
                assert_eq!(stack.pop(), 30);
                assert_eq!(stack.len(), 1);
                stack.push(45);
                assert_eq!(stack.pop(), 45);
                stack.push(3);
                stack.push(5);
                assert_eq!(stack.len(), 3);
                assert_eq!(stack.pop(), 5);
                assert_eq!(stack.pop(), 3);
                assert_eq!(stack.pop(), 15);
            }
        }
    }
}

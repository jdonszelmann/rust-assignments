use weblab::weblab;

#[weblab(programming_assignment)]
/// In this assignment, you will learn how to interact with iterators in Rust.
/// Implement all of the functions.
///
/// You should use only iterators to solve this assignment. No explicit loops (`for`, `while`, `loop`) and let statements are allowed or needed.
#[weblab(title = "Exercises")]
#[weblab(weight=3)]
#[weblab(check="no explicit loops were used (only iterators). Using if statements is allowed.")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        /// Calculate the sum of this list
        pub fn sum(vec: Vec<i64>) -> i64 {
            vec.iter().sum()
        }

        /// Count how often the number 42 appears in this list
        pub fn counter_42(vec: Vec<i64>) -> usize {
            vec.iter().filter(|v| **v == 42).count()
        }

        /// Find the index of the third time that the number 42 appears in this list.
        /// Return `None` when the list does not contain three 42s.
        ///
        /// Example 1: `[0,1,42,3,42,5,6,42,8,9]` -> Some(7)
        ///
        /// Example 2: `[0,1,42,3,42,5,6,7,8,9]` -> None
        pub fn find_third_42(vec: Vec<i64>) -> Option<usize> {
            vec.iter()
                .enumerate()
                .filter(|v| *v.1 == 42)
                .map(|v| v.0)
                .nth(2)
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// Calculate the sum of this list
        pub fn sum(vec: Vec<i64>) -> i64 {
            todo!()
        }

        /// Count how often the number 42 appears in this list
        pub fn counter_42(vec: Vec<i64>) -> usize {
            todo!()
        }

        /// Find the index of the third time that the number 42 appears in this list.
        /// Return `None` when the list does not contain three 42s.
        ///
        /// Example 1: `[0,1,42,3,42,5,6,42,8,9]` -> Some(7)
        ///
        /// Example 2: `[0,1,42,3,42,5,6,7,8,9]` -> None
        pub fn find_third_42(vec: Vec<i64>) -> Option<usize> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::solution_only;

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
                sum1: assert_eq!(sum(vec![]), 0);
                sum2: assert_eq!(sum(vec![69]), 69);
                sum3: assert_eq!(sum(vec![69,42,-1]), 69+42-1);

                count1: assert_eq!(counter_42(vec![69]), 0);
                count2: assert_eq!(counter_42(vec![69,42,-1]), 1);
                count3: assert_eq!(counter_42(vec![69,42,-1, 42]), 2);
                count4: assert_eq!(counter_42(vec![42, 69,42,-1]), 2);
                count5: assert_eq!(counter_42(vec![69,42,-1,42,42,42,42,42]), 6);

                third1: assert_eq!(find_third_42(vec![]), None);
                third2: assert_eq!(find_third_42(vec![69]), None);
                third3: assert_eq!(find_third_42(vec![69,42,-1]), None);
                third4: assert_eq!(find_third_42(vec![69,42,-1, 42]), None);
                third5: assert_eq!(find_third_42(vec![42, 69,42,-1]), None);
                third6: assert_eq!(find_third_42(vec![69,42,-1,42,42,42,42,42]), Some(4));
                third7: assert_eq!(find_third_42(vec![69,42,-1,42,0,0,42,8]), Some(6));
            }
        }
    }
}

use weblab::weblab;

#[weblab(programming_assignment)]
/// In this assignment, you will learn to work with the Vec type.
/// A Vec is a resizable array, which can store any type of value. For example, a `Vec<i64>` is a list of `i64` values.
/// You should implement each function according to the description provided in the comment above it.
///
/// You are not allowed to change any function signatures.
#[weblab(title = "All about vecs")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        /// Insert the value `3` at the end of the given Vec.
        pub fn insert_three(vec: Vec<i64>) -> Vec<i64> {
            template_only! { todo!() }
            solution_only! {
                let mut vec = vec;
                vec.push(3);
                vec
            }
        }

        /// Insert the value `3` at the end of the given Vec.
        pub fn insert_three_mut_ref(vec: &mut Vec<i64>) {
            template_only! { todo!() }
            solution_only! {
                vec.push(3);
            }
        }

        /// Return the length of the Vec
        pub fn length(vec: &Vec<i64>) -> usize {
            template_only! { todo!() }
            solution_only! {
                vec.len()
            }
        }

        /// Return the length of the Vec
        /// Note that the return type is now `i64`, don't change it.
        pub fn length_i64(vec: &Vec<i64>) -> i64 {
            template_only! { todo!() }
            solution_only! {
                vec.len() as i64
            }
        }

        /// Return the sum of the Vec.
        pub fn sum_elements(vec: &Vec<i64>) -> i64 {
            template_only! { todo!() }
            solution_only! {
                let mut sum = 0;
                for elem in vec {
                    sum += *elem;
                }
                sum
                //Or just vec.iter().sum()
            }
        }

        /// Return the sum of all values that are even in the Vec.
        /// For example, the sum of all the even elements in `[3,4,8,2,5,0]` = 4 + 8 + 2 + 0 = 14
        pub fn sum_even_values(vec: &Vec<i64>) -> i64 {
            template_only! { todo!() }
            solution_only! {
                let mut sum = 0;
                for elem in vec {
                    if elem % 2 == 0 {
                        sum += *elem;
                    }
                }
                sum
                //Or just vec.iter().filter(|v| *v % 2 == 0).sum()
            }
        }

        /// Return the sum of all values that are at an even position in the vec.
        /// This means the element at position 0 (the first element), position 2 (the third element), etc...
        /// For example, the sum of all the even elements in `[3,4,8,2,5,0]` = 3 + 8 + 5 = 16
        pub fn sum_even_indices(vec: &Vec<i64>) -> i64 {
            template_only! { todo!() }
            solution_only! {
                let mut sum = 0;
                #[allow(clippy::needless_range_loop)]
                for i in 0..vec.len() {
                    if i % 2 == 0 {
                        sum += vec[i];
                    }
                }
                sum
                //Or just vec.iter().enumerate().filter(|(i, _)| *i % 2 == 0).map(|(_, v)| v).sum()
            }
        }

        /// Add 1 to each value in the vec. Note that the vec is passed in by mutable reference.
        /// For example, the vec `[3,4,8,2,5,0]` should be changed to `[4,5,9,3,6,1]`
        pub fn add_1_to_all_mut_ref(vec: &mut Vec<i64>) {
            template_only! { todo!() }
            solution_only! {
                for elem in vec {
                    *elem += 1;
                }
            }
        }

        /// Add 1 to each value in the vec. Note that the vec is given as an argument by value, and needs to be returned.
        /// This function should be implemented by calling the `add_1_to_all_v1` function above.
        pub fn add_1_to_all(vec: Vec<i64>) -> Vec<i64> {
            template_only! { todo!() }
            solution_only! {
                let mut vec = vec;
                add_1_to_all_mut_ref(&mut vec);
                vec
            }
        }
    }

    #[weblab(test_template)]
    mod test_template {}

    #[weblab(test)]
    mod test {
        use super::solution::*;

        #[test]
        pub fn test_insert_three() {
            assert_eq!(insert_three(vec![]), vec![3]);
            assert_eq!(insert_three(vec![2]), vec![2, 3]);
            assert_eq!(insert_three(vec![3]), vec![3, 3]);
            assert_eq!(
                insert_three(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 3]
            );
        }

        #[test]
        pub fn test_insert_three_mut_ref() {
            let mut vec = vec![];
            insert_three_mut_ref(&mut vec);
            assert_eq!(vec, vec![3]);

            let mut vec = vec![2];
            insert_three_mut_ref(&mut vec);
            assert_eq!(vec, vec![2, 3]);

            let mut vec = vec![3];
            insert_three_mut_ref(&mut vec);
            assert_eq!(vec, vec![3, 3]);

            let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
            insert_three_mut_ref(&mut vec);
            assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 3]);
        }

        #[test]
        pub fn test_length() {
            assert_eq!(length(&vec![]), 0);
            assert_eq!(length(&vec![1]), 1);
            assert_eq!(length(&vec![1, 2]), 2);
            assert_eq!(length(&vec![1, 2, 3]), 3);
            assert_eq!(length(&vec![1, 2, 3, 4, 5]), 5);
        }

        #[test]
        pub fn test_length_i64() {
            assert_eq!(length_i64(&vec![]), 0);
            assert_eq!(length_i64(&vec![1]), 1);
            assert_eq!(length_i64(&vec![1, 2]), 2);
            assert_eq!(length_i64(&vec![1, 2, 3]), 3);
            assert_eq!(length_i64(&vec![1, 2, 3, 4, 5]), 5);
        }

        #[test]
        pub fn test_sum_elements() {
            assert_eq!(sum_elements(&vec![3, 4, 8, 2, 5, 0]), 22);
            assert_eq!(sum_elements(&vec![]), 0);
            assert_eq!(sum_elements(&vec![19, -19, 3]), 3);
            assert_eq!(sum_elements(&vec![-99]), -99);
            assert_eq!(sum_elements(&vec![10081, 122, -123]), 10080);
        }

        #[test]
        pub fn test_sum_even_elements() {
            assert_eq!(sum_even_values(&vec![3, 4, 8, 2, 5, 0]), 14);
            assert_eq!(sum_even_values(&vec![]), 0);
            assert_eq!(sum_even_values(&vec![19, -19, 3]), 0);
            assert_eq!(sum_even_values(&vec![-99]), 0);
            assert_eq!(sum_even_values(&vec![10080, 122, -123]), 10202);
            assert_eq!(sum_even_values(&vec![188]), 188);
        }

        #[test]
        pub fn test_sum_even_indices() {
            assert_eq!(sum_even_indices(&vec![3, 4, 8, 2, 5, 0]), 16);
            assert_eq!(sum_even_indices(&vec![]), 0);
            assert_eq!(sum_even_indices(&vec![19, -19, 3]), 22);
            assert_eq!(sum_even_indices(&vec![-99]), -99);
            assert_eq!(sum_even_indices(&vec![10080, 122, -123]), 9957);
            assert_eq!(sum_even_indices(&vec![188]), 188);
        }

        #[test]
        pub fn test_add_1_to_all_mut_ref() {
            let mut vec = vec![1, 2, 3, 4, 5];
            add_1_to_all_mut_ref(&mut vec);
            assert_eq!(vec, vec![2, 3, 4, 5, 6]);

            let mut vec: Vec<i64> = vec![];
            add_1_to_all_mut_ref(&mut vec);
            assert_eq!(vec, Vec::<i64>::new());

            let mut vec = vec![-88, 77, 23];
            add_1_to_all_mut_ref(&mut vec);
            assert_eq!(vec, vec![-87, 78, 24]);

            let mut vec = vec![1212190, 111110];
            add_1_to_all_mut_ref(&mut vec);
            assert_eq!(vec, vec![1212191, 111111]);
        }

        #[test]
        pub fn test_add_1_to_all() {
            assert_eq!(add_1_to_all(vec![1, 2, 3, 4, 5]), vec![2, 3, 4, 5, 6]);
            assert_eq!(add_1_to_all(vec![]), Vec::<i64>::new());
            assert_eq!(add_1_to_all(vec![-88, 77, 23]), vec![-87, 78, 24]);
            assert_eq!(
                add_1_to_all(vec![1212190, 111110]),
                vec![1212191, 111111]
            );
        }
    }
}

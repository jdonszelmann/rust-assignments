use weblab::weblab;

#[weblab(programming_assignment)]
/// Binary search is an algorithm that is used to quickly find an element in a sorted list.
/// To be precise, the index of the value in the list is returned.
/// For more information, see: https://en.wikipedia.org/wiki/Binary_search_algorithm#Algorithm
///
/// A short summary of the algorithm:
///
/// - We keep track of a `min` and `max`, the minimum and maximum possible index
/// - We check the value at the index precisely in the middle of `min` and `max`.
///     - If the value at the index is equal to the value we are searching for, return the index
///     - If the value at the index is lower than the value we are searching for, we know the value is in the range `[min, middle)`
///     - If the value at the index is higher than the value we are searching for, we know the value is in the range `(middle, max]`
/// - We update the bounds to conform to the new range, and rerun. If the range is ever empty, we know the value we are searching for is not in the list. In this case, return None.
#[weblab(title = "Binary Search")]
#[weblab(weight = 3)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::cmp::Ordering;

        pub fn binary_search(list: &Vec<i64>, value: i64) -> Option<i64> {
            let mut min = 0i64;
            let mut max = list.len() as i64 - 1;

            while min <= max {
                let middle = (min + max) / 2;
                match list[middle as usize].cmp(&value) {
                    Ordering::Less => {
                        min = middle + 1;
                    }
                    Ordering::Equal => return Some(middle),
                    Ordering::Greater => {
                        max = middle - 1;
                    }
                }
            }
            None
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    #[allow(clippy::ptr_arg)]
    mod solution_template {
        /// list is guaranteed to be sorted
        pub fn binary_search(list: &Vec<i64>, value: i64) -> Option<i64> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        template_only! {
            #[test]
            fn find_is_there() {
                let vec = vec![0,1,2,3,4,5,6,7,8];
                assert_eq!(Some(7), binary_search(&vec, 7));
            }

            #[test]
            fn find_is_not_there() {
                let vec = vec![0,1,2,3,4,5,6,8,9];
                assert_eq!(None, binary_search(&vec, 7));
            }
        }

        solution_only! {
            #[test]
            fn find_full_1() {
                let vec = vec![0,1,2,3,4,5,6,7,8];
                assert_eq!(Some(0), binary_search(&vec, 0));
                assert_eq!(Some(1), binary_search(&vec, 1));
                assert_eq!(Some(2), binary_search(&vec, 2));
                assert_eq!(Some(3), binary_search(&vec, 3));
                assert_eq!(Some(4), binary_search(&vec, 4));
                assert_eq!(Some(5), binary_search(&vec, 5));
                assert_eq!(Some(6), binary_search(&vec, 6));
                assert_eq!(Some(7), binary_search(&vec, 7));
                assert_eq!(None, binary_search(&vec, -1));
                assert_eq!(None, binary_search(&vec, 9));
                assert_eq!(None, binary_search(&vec, 1898));

            }

            #[test]
            fn find_full_2() {
                let vec = vec![0,2,4,6,8,10,12,14,16];
                assert_eq!(Some(0), binary_search(&vec, 0));
                assert_eq!(Some(1), binary_search(&vec, 2));
                assert_eq!(Some(2), binary_search(&vec, 4));
                assert_eq!(Some(3), binary_search(&vec, 6));
                assert_eq!(Some(4), binary_search(&vec, 8));
                assert_eq!(Some(5), binary_search(&vec, 10));
                assert_eq!(Some(6), binary_search(&vec, 12));
                assert_eq!(Some(7), binary_search(&vec, 14));
                assert_eq!(Some(8), binary_search(&vec, 16));

                assert_eq!(None, binary_search(&vec, -1));
                assert_eq!(None, binary_search(&vec, 1));
                assert_eq!(None, binary_search(&vec, 3));
                assert_eq!(None, binary_search(&vec, 5));
                assert_eq!(None, binary_search(&vec, 7));
                assert_eq!(None, binary_search(&vec, 9));
                assert_eq!(None, binary_search(&vec, 11));
                assert_eq!(None, binary_search(&vec, 13));
                assert_eq!(None, binary_search(&vec, 15));
                assert_eq!(None, binary_search(&vec, 17));
            }

            #[test]
            fn edgecase_1() {
                let vec = vec![];
                assert_eq!(None, binary_search(&vec, 0));
            }

            #[test]
            fn edgecase_2() {
                let vec = vec![0];
                assert_eq!(None, binary_search(&vec, 1));
                assert_eq!(Some(0), binary_search(&vec, 0));
            }

            #[test]
            fn edgecase_3() {
                let vec = vec![0,2];
                assert_eq!(None, binary_search(&vec, 1));
                assert_eq!(Some(0), binary_search(&vec, 0));
                assert_eq!(Some(1), binary_search(&vec, 2));
            }
        }
    }
}

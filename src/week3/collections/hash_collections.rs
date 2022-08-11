use weblab::weblab;

#[weblab(programming_assignment)]
/// The HashSet is a datastructure that is a set of elements (a list with no duplicates).
/// It is very fast to add and remove elements from this set.
/// To be precise, the speed is O(1), meaning that adding and remove elements and checking if elements are in the Set does not get slower as the amount of elements in the set increases.
/// This is in contrast with a Vec, where removing a specific element would be O(n), meaning that the time needed to remove an element grows linearly with the size of the vec.
///
/// A HashMap is a datastructure that is a Map, meaning a set of pairs (Key, Value). Each key is unique, just like in a HashSet, but each key has an associated value.
/// HashMaps have a `get` method to obtain the value associated with a specific key, `insert` to add a new (Key, Value) pair and `remove` to remove a key and its associated value.
/// The methods have the same speed as a HashSet, O(1).
///
/// Given are 3 functions to implement. You have to implement these functions efficiently, using a `HashSet` or `HashMap`.
#[weblab(title = "Hash Collections")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use itertools::Itertools;
        use std::collections::{HashMap, HashSet};
        use weblab::{solution_only, template_only};

        /// Return whether there are any duplicate elements in this vec.
        pub fn has_duplicates(vec: Vec<u64>) -> bool {
            solution_only! {
                let mut set: HashSet<u64> = HashSet::new();
                for elem in vec {
                    if set.contains(&elem) {
                        return true;
                    }
                    set.insert(elem);
                }
                false
            }
            template_only! {
                todo!()
            }
        }

        /// Find the most common element in this Vec.
        /// If multiple elements are equally common, return any of them.
        /// If the vec is empty, panic.
        pub fn most_common_element(vec: Vec<u64>) -> u64 {
            solution_only! {
                let mut map: HashMap<u64, u64> = HashMap::new();
                for elem in vec {
                    *map.entry(elem).or_insert(0) += 1;
                }
                *map.iter().max_by_key(|x| x.1).expect("Vec not empty").0
            }
            template_only! {
                todo!()
            }
        }

        /// Provided is a vec with all numbers from `x` to `y` inclusive, except one.
        /// Find the number that is missing.
        ///
        /// For example, x = `3`, y = `13`, vec = `[8,3,9,13,12,7,4,5,10,11]`
        /// The missing number is 6.
        pub fn which_number_is_missing(vec: Vec<u64>, x: u64, y: u64) -> u64 {
            solution_only! {
                let mut set: HashSet<u64> = HashSet::from_iter(x..=y);
                for v in vec {
                    set.remove(&v);
                }
                set.into_iter().next().unwrap()
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

        template_only! {
            #[test]
            fn test_has_duplicates() {
                assert!(has_duplicates(vec![0,1,0]));
            }

            #[test]
            fn test_most_common_element() {
                assert_eq!(most_common_element(vec![0,1,0]), 0);
            }

            #[test]
            fn test_missing_number() {
                assert_eq!(which_number_is_missing(vec![8,3,9,13,12,7,4,5,10,11], 3, 13), 6);
            }
        }

        solution_only! {
            #[test]
            fn test_has_duplicates2() {
                assert!(has_duplicates(vec![0,1,0]));
                assert!(has_duplicates(vec![0,1,0,0]));
                assert!(has_duplicates(vec![0,1,0,2,3,4]));
                assert!(has_duplicates(vec![22,1,0,9,3,4,5,22,1]));

                assert!(!has_duplicates(vec![22,1,0,9,3,4,5]));
                assert!(!has_duplicates(vec![0,3,5,9,2,4,6,8,10]));
                assert!(!has_duplicates(vec![0,u64::MAX]));
            }

            #[test]
            fn test_has_duplicates3() {
                assert!(has_duplicates(vec![0; 1000000]));
                assert!(!has_duplicates(Vec::from_iter(0..1000000)));
            }

            #[test]
            fn test_most_common_element2() {
                assert_eq!(most_common_element(vec![0,1,0]), 0);
                assert_eq!(most_common_element(vec![0,1,0,0]), 0);
                assert_eq!(most_common_element(vec![0,1,0,2,3,4]), 0);

                let v = most_common_element(vec![22,1,0,9,3,4,5,22,1]);
                assert!(v == 1 || v == 22);

                assert_eq!(most_common_element(vec![0,u64::MAX, u64::MAX]), u64::MAX);
            }

            #[test]
            fn test_most_common_element3() {
                assert_eq!(most_common_element(vec![0; 1000000]), 0);
                let v = most_common_element(Vec::from_iter(0..1000000));
                assert!(v < 1000000);
            }

            #[test]
            fn test_missing_number2() {
                assert_eq!(which_number_is_missing(vec![8,3,9,13,12,7,4,5,10,11], 3, 13), 6);
                assert_eq!(which_number_is_missing(vec![8,3,6,13,12,7,4,5,10,11], 3, 13), 9);
            }

            #[test]
            fn test_missing_number3() {
                assert_eq!(which_number_is_missing(Vec::from_iter(0..1000000), 0, 1000000), 1000000);
            }
        }
    }
}

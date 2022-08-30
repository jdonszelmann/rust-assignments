use weblab::weblab;

#[weblab(programming_assignment)]
/// A Set is a collection of elements, without duplicates. To create a set,
/// we could use a Vec. Then, every time we inserted we would need to check
/// the new element against all existing elements to see if it was already present,
/// in which case we don't insert the new element.
///
/// However, if the set becomes bigger, looking for duplicates takes longer and longer.
/// We say that the complexity of inserting is `O(n)`, meaning that the time it takes
/// grows linearly with the number of elements in the set.
///
/// A hashset is a set datastructure that can insert (and look if an element is present)
/// in `O(1)` time. This means that every lookup takes the same amount of time, regardless
/// of the number of elements in the set.
///
/// A HashMap is a datastructure that is a Map, meaning a set of pairs (Key, Value). Each key is unique, just like in a HashSet, but each key has an associated value.
/// HashMaps have a `get` method to obtain the value associated with a specific key, `insert` to add a new (Key, Value) pair and `remove` to remove a key and its associated value.
/// The methods have the same speed as a HashSet, O(1).
///
/// Given are 3 functions to implement. You have to implement these functions efficiently, using a `HashSet` or `HashMap`.
#[weblab(title = "Hashed collections")]
#[weblab(weight=2)]
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
        /// If the vec is empty, return None.
        pub fn most_common_element(vec: Vec<u64>) -> Option<u64> {
            solution_only! {
                let mut map: HashMap<u64, u64> = HashMap::new();
                for elem in vec {
                    *map.entry(elem).or_insert(0) += 1;
                }

                Some(*map.iter().max_by_key(|x| x.1)?.0)
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
        use weblab::solution_only;

        #[test]
        fn test_has_duplicates() {
            assert!(has_duplicates(vec![0,1,0]));
        }

        #[test]
        fn test_most_common_element() {
            assert_eq!(most_common_element(vec![0,1,0]), Some(0));
        }

        #[test]
        fn test_missing_number() {
            assert_eq!(which_number_is_missing(vec![8,3,9,13,12,7,4,5,10,11], 3, 13), 6);
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
                dup1: assert!(has_duplicates(vec![0,1,0]));
                dup2: assert!(has_duplicates(vec![0,1,0,0]));
                dup3: assert!(has_duplicates(vec![0,1,0,2,3,4]));
                dup4: assert!(has_duplicates(vec![22,1,0,9,3,4,5,22,1]));
                dup5: assert!(!has_duplicates(vec![22,1,0,9,3,4,5]));
                dup6: assert!(!has_duplicates(vec![0,3,5,9,2,4,6,8,10]));
                dup7: assert!(!has_duplicates(vec![0,u64::MAX]));
                dup8: assert!(has_duplicates(vec![0; 1000000]));
                dup9: assert!(!has_duplicates(Vec::from_iter(0..1000000)));
            }

            test_each! {
                com1: assert_eq!(most_common_element(vec![0,1,0]), Some(0));
                com2: assert_eq!(most_common_element(vec![0,1,0,0]), Some(0));
                com3: assert_eq!(most_common_element(vec![0,1,0,2,3,4]), Some(0));
                com4: {
                    let v = most_common_element(vec![22,1,0,9,3,4,5,22,1]);
                    assert!(v == Some(1) || v == Some(22))
                };
                com5: assert_eq!(most_common_element(vec![0,u64::MAX, u64::MAX]), Some(u64::MAX));
                com6: assert_eq!(most_common_element(vec![]), None);
                com7: assert_eq!(most_common_element(vec![0; 1000000]), Some(0));

                com8: {
                    let v = most_common_element(Vec::from_iter(0..1000000)).unwrap();
                    assert!(v < 1000000);
                }
            }

            test_each! {
                mis1: assert_eq!(which_number_is_missing(vec![8,3,9,13,12,7,4,5,10,11], 3, 13), 6);
                mis2: assert_eq!(which_number_is_missing(vec![8,3,6,13,12,7,4,5,10,11], 3, 13), 9);
                mis3: assert_eq!(which_number_is_missing(Vec::from_iter(0..1000000), 0, 1000000), 1000000);
            }
        }
    }
}

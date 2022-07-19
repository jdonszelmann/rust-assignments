use weblab::weblab;

#[weblab(programming_assignment)]
/// In this exercise, you need to implement bubble sort.
/// Bubble sort works the following way:
///
/// 1. Iterate over the entire list from left to right, comparing the adjacent elements, and swap them if they are not in the correct order.
/// 2. After doing this for one iteration, the largest element in the list will have moved to the end.
/// 3. Repeat the process for the sublist that remains unsorted (so, without the last element)
///
/// Do not just call the stdlib `.sort()` function.
#[weblab(title = "Bubble Sort")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn bubble_sort(list: &mut Vec<i64>) {
            for end in 0..list.len() {
                for cur in 0..list.len() - end - 1 {
                    if list[cur] > list[cur + 1] {
                        list.swap(cur, cur + 1);
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    #[allow(clippy::ptr_arg)]
    mod solution_template {
        pub fn bubble_sort(list: &mut Vec<i64>) {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        solution_only! {
            #[test]
            fn test_simple() {
                let mut vec = vec![1,2,3];
                bubble_sort(&mut vec);
                assert_eq!(vec, vec![1,2,3]);
            }
        }
    }
}

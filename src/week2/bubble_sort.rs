use weblab::weblab;

#[weblab(programming_assignment)]
/// In this exercise, you need to implement bubble sort.
/// Bubble sort works the following way:
///
/// 1. Iterate over the entire list from left to right, comparing the adjacent elements, and swap them if they are not in the correct order.
/// 2. After doing this for one iteration, the largest element in the list will have moved to the end.
/// 3. Repeat the process for the sublist that remains unsorted (so, without the last element)
///
/// The wikipedia page contains a more in-depth explanation together with an animated example.
/// https://en.wikipedia.org/wiki/Bubble_sort
///
/// You *are* allowed to use the swap method on Vec
/// You *are not* allowed to just call the stdlib `.sort()` function.
#[weblab(title = "Bubble Sort")]
#[weblab(check = "manually implements bubble sort")]
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
            macro_rules! sort_test {
                ($($name:ident : $a: expr => $b: expr);* $(;)?) => {
                    $(
                        #[test]
                        fn $name() {
                            let mut v = $a;
                            bubble_sort(&mut v);
                            assert_eq!(v, $b);
                        }
                    )*
                };
            }

            sort_test!{
                s1: vec![1, 2, 3]=> vec![1, 2, 3];
                s2: vec![3, 2, 1]=> vec![1, 2, 3];
                s3: (0..1000).collect::<Vec<_>>() => (0..1000).collect::<Vec<_>>();
                s4: (0..1000).rev().collect::<Vec<_>>() => (0..1000).collect::<Vec<_>>();
            }
        }
    }
}

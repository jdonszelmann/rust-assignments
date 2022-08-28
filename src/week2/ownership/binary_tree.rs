use weblab::weblab;

#[weblab(programming_assignment)]
/// A binary search tree is a data structure to search through sorted data.
/// An implementation of a binary search tree enum is given.
///
/// In a binary search tree, each node has either two or no children. If a node has two children,
/// one of the children only contains items *smaller* than the element in that node, and the other
/// only contains items *larger* than the element in that node. This property ensures that searching
/// whether or not an element is in the tree takes an amount of time correlated with the logarithm of the number
/// of elements in the tree (if the tree is balanced).
///
/// You can read more about them [here, on Wikipedia](https://en.wikipedia.org/wiki/Binary_search_tree)
/// Implement the empty methods on the tree.
///
/// The problems here are most easily solved by using recursion. Look at the implementation of `height`.
/// We know that the height of a `Leaf` node is always 0 because it has no children.
/// We also know that the height of any other node depends on the height of its children. Specifically, it's one more than
/// the highest of the two children. Those two properties are enough to implement the `height` function.
#[weblab(title = "Binary Search Tree")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        /// A binary tree, generic over T. It means that
        /// regardless of what T is, you can always store it in a binary tree.
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum BinaryTree<T> {
            Node {
                value: T,
                larger: Box<Self>,
                smaller: Box<Self>,
            },
            Leaf,
        }

        /// These methods always exist, regardless of what T is.
        impl<T> BinaryTree<T> {
            /// Get the height of the binary tree. That is, the largest distance between one of the leafs and the root.
            /// Note that the height of a node is 1 more than the largest of the two child heights.
            pub fn height(&self) -> usize {
                match self {
                    BinaryTree::Node {
                        larger, smaller, ..
                    } => 1 + larger.height().max(smaller.height()),
                    BinaryTree::Leaf => 0,
                }
            }

            // Return the number of non-leaf nodes in the tree. You can use recursion to solve this problem.
            pub fn size(&self) -> usize {
                solution_only! {
                    match self {
                        BinaryTree::Node { smaller, larger, .. } => smaller.size() + larger.size() + 1,
                        BinaryTree::Leaf => 0
                    }
                }
                template_only! {
                    todo!()
                }
            }
        }

        /// These methods that only exist if T implements `Ord`. That means, that if we have
        /// ```
        /// let a: T;
        /// let b: T;
        /// ```
        ///
        /// We can compare a and b using the normal comparison operators (`<` and `>` and `==` etc.)
        /// We need to compare elements to know if they need to be placed on the small or large side of a node.
        impl<T: Ord> BinaryTree<T> {
            /// Insert a value into a binary tree. Return a new tree, with the value added.
            /// You can ignore and disregard duplicate values in the tree.
            pub fn insert(self, to_insert: T) -> Self {
                solution_only! {
                    match self {
                        BinaryTree::Node { larger, smaller, value, ..} if to_insert < value => {
                            BinaryTree::Node {
                                smaller: Box::new(smaller.insert(to_insert)),
                                larger,
                                value
                            }
                        }
                        BinaryTree::Node { larger, smaller, value, ..} if to_insert > value => {
                            BinaryTree::Node {
                                smaller,
                                larger: Box::new(larger.insert(to_insert)),
                                value
                            }
                        }
                        a@BinaryTree::Node {..} => a,
                        BinaryTree::Leaf => BinaryTree::Node {
                            value: to_insert,
                            larger: Box::new(BinaryTree::Leaf),
                            smaller: Box::new(BinaryTree::Leaf)
                        }
                    }
                }
                template_only! {
                    todo!()
                }
            }

            /// Returns whether or not the binary tree contains an element v.
            /// Note that because the tree is sorted, you do not need to search through the entire tree. You can compare
            /// the element we're searching for with elements in the tree, to find an answer faster.
            pub fn contains(&self, v: &T) -> bool {
                solution_only! {
                    match self {
                        BinaryTree::Leaf => false,
                        BinaryTree::Node { value, .. } if value == v => true,
                        BinaryTree::Node { smaller, larger, value } => {
                            if v < value {
                                smaller.contains(v)
                            } else {
                                larger.contains(v)
                            }
                        }
                    }
                }
                template_only! {
                    todo!()
                }
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use itertools::Itertools;
        use weblab::{solution_only, template_only};

        solution_only! {
            fn leaf<T>() -> BinaryTree<T> {
                BinaryTree::Leaf
            }

            fn node<T>(t: T, s: BinaryTree<T>, l: BinaryTree<T>) -> BinaryTree<T> {
                BinaryTree::Node {
                    value: t,
                    smaller: Box::new(s),
                    larger: Box::new(l),
                }
            }

            #[test]
            fn test_size_skel() {
                assert_eq!(node(1, node(0, leaf(), leaf()), node(2, leaf(), leaf())).size(), 3);
            }

            #[test]
            fn test_size_1() {
                assert_eq!(leaf::<usize>().size(), 0);
            }

            #[test]
            fn test_size_2() {
                assert_eq!(node(0, leaf(), leaf()).size(), 1);
            }

            #[test]
            fn test_size_3() {
                assert_eq!(node(0, node(1, leaf(), leaf()), node(2, leaf(), leaf())).size(), 3);
            }

            #[test]
            fn test_size_7() {
                assert_eq!(
                    node(4, node(2, node(1, leaf(), leaf()), node(3, leaf(), leaf())), node(6, node(5, leaf(), leaf()), node(7, leaf(), leaf())))
                    .size(),
                    7
                );
            }

            #[test]
            fn test_insert_1() {
                assert_eq!(
                    node(5, node(4, node(3, leaf(), leaf()), leaf()), leaf()),
                    leaf().insert(5).insert(4).insert(3)
                );
            }

            #[test]
            fn test_insert_2() {
                assert_eq!(
                    node(5, node(4, leaf(), leaf()), node(52, leaf(), leaf())),
                    leaf().insert(5).insert(4).insert(5).insert(52)
                );
            }

            #[test]
            fn test_insert_3() {
                assert_eq!(
                    node(10, node(4, node(3, leaf(), leaf()), node(5, leaf(), leaf())), node(12, node(11, leaf(), leaf()), leaf())),
                    leaf().insert(10).insert(4).insert(12).insert(11).insert(5).insert(3)
                )
            }

            #[test]
            fn test_contains_0() {
                assert!(!leaf().contains(&3))
            }

            #[test]
            fn test_contains_1() {
                assert!(leaf().insert(3).insert(1).insert(9).insert(5).contains(&3));
            }

            #[test]
            fn test_contains_2() {
                assert!(!leaf().insert(3).insert(1).insert(9).insert(5).contains(&7));
            }
        }
    }
}

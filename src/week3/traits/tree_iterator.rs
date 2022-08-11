use weblab::weblab;

#[weblab(programming_assignment)]
/// You are given an implementation of a binary search tree. Some operations on it are already defined.
/// The tree already implements IntoIterator.
/// Implement the `Iterator` trait for the tree.
#[weblab(title = "Tree iterator")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::solution_only;

        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum BinaryTree<T> {
            Node {
                value: T,
                larger: Box<Self>,
                smaller: Box<Self>,
            },
            Leaf,
        }

        impl<T> BinaryTree<T> {
            pub fn new() -> Self {
                Self::Leaf
            }

            pub fn height(&self) -> usize {
                match self {
                    BinaryTree::Node { larger, smaller, .. } => {
                        1 + larger.height().max(smaller.height())
                    },
                    BinaryTree::Leaf => 0
                }
            }

            pub fn size(&self) -> usize {
                match self {
                    BinaryTree::Node { smaller, larger, .. } => smaller.size() + larger.size() + 1,
                    BinaryTree::Leaf => 0
                }
            }
        }

        impl<T: PartialOrd> BinaryTree<T> {
            pub fn insert(self, to_insert: T) -> Self {
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

            pub fn contains(&self, v: &T) -> bool {
                match self {
                    BinaryTree::Leaf => false,
                    BinaryTree::Node { value, ..} if value == v => true,
                    BinaryTree::Node { smaller, larger, value } => {
                        if v < value {
                            smaller.contains(v)
                        } else {
                            larger.contains(v)
                        }
                    }
                }
            }
        }
    }

    #[weblab(test)]
    mod test {
        use itertools::Itertools;
        use super::solution::*;
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
            fn test_bt_intact() {
                assert_eq!(node(1, node(0, leaf(), leaf()), node(2, leaf(), leaf())).size(), 3);
                assert_eq!(leaf::<usize>().size(), 0);
                assert_eq!(node(0, leaf(), leaf()).size(), 1);
                assert_eq!(node(0, node(1, leaf(), leaf()), node(2, leaf(), leaf())).size(), 3);
                assert_eq!(
                    node(4, node(2, node(1, leaf(), leaf()), node(3, leaf(), leaf())), node(6, node(5, leaf(), leaf()), node(7, leaf(), leaf())))
                    .size(),
                    7
                );
                assert_eq!(
                    node(5, node(4, node(3, leaf(), leaf()), leaf()), leaf()),
                    leaf().insert(5).insert(4).insert(3)
                );
                assert_eq!(
                    node(5, node(4, leaf(), leaf()), node(52, leaf(), leaf())),
                    leaf().insert(5).insert(4).insert(5).insert(52)
                );
                assert_eq!(
                    node(10, node(4, node(3, leaf(), leaf()), node(5, leaf(), leaf())), node(12, node(11, leaf(), leaf()), leaf())),
                    leaf().insert(10).insert(4).insert(12).insert(11).insert(5).insert(3)
                );
                assert!(!leaf().contains(&3));
                assert!(leaf().insert(3).insert(1).insert(9).insert(5).contains(&3));
                assert!(!leaf().insert(3).insert(1).insert(9).insert(5).contains(&7));
            }
        }
    }
}

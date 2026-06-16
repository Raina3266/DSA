mod codility_word_machine;
mod google_find_the_longest_word;
mod graph;
mod linked_list;
mod q100_same_tree;
mod q121_best_time_to_buy_and_sell_stock;
mod q128_longest_consecutive_sequence;
mod q15_three_sum;
mod q1_two_sum;
mod q146_lru_cache;
mod q20_valid_parentheses;
mod q207_course_schedule;
mod q217_contain_duplicate;
mod q226_invert_binary_tree;
mod q230_kth_smallest_element_in_a_bst;
mod q238_product_of_array_except_self;
mod q242_valid_anagram;
mod q2571_minimum_operations_to_reduce_an_integer_to_0;
mod q572_subtree_of_another_tree;
mod q5_longest_palindromic_substring;
mod q647_palindromic_substrings;
mod tree;
mod hacker_graph_1;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new() -> Self {
        TreeNode {
            value: 1,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: Some(Box::new(TreeNode {
                    value: 4,
                    left: Some(Box::new(TreeNode {
                        value: 8,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        value: 9,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(TreeNode {
                    value: 5,
                    left: Some(Box::new(TreeNode {
                        value: 10,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 3,
                left: Some(Box::new(TreeNode {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
        }
    }

    fn new_leaf(value: i32) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
    pub fn new_binary_tree() -> Self {
        // Visual Structure of the tree we are building:
        //          5
        //        /   \
        //       3     8
        //      / \   / \
        //     2   4 7   9
        //    /     /     \
        //   1     6      10

        // 1. Create the bottom-most leaves first
        let node1 = Self::new_leaf(1);
        let node4 = Self::new_leaf(4);
        let node6 = Self::new_leaf(6);
        let node10 = Self::new_leaf(10);

        // 2. Build the next level up, attaching the leaves
        let node2 = Some(Box::new(TreeNode {
            value: 2,
            left: node1,
            right: None,
        }));
        let node7 = Some(Box::new(TreeNode {
            value: 7,
            left: node6,
            right: None,
        }));
        let node9 = Some(Box::new(TreeNode {
            value: 9,
            left: None,
            right: node10,
        }));

        // 3. Build the level below the root
        let node3 = Some(Box::new(TreeNode {
            value: 3,
            left: node2,
            right: node4,
        }));
        let node8 = Some(Box::new(TreeNode {
            value: 8,
            left: node7,
            right: node9,
        }));

        // 4. Finally, create and return the root
        TreeNode {
            value: 5,
            left: node3,
            right: node8,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

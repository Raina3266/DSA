// In a Binary Search Tree, everything to the left of a node is smaller, and everything to the right is larger.

use std::collections::VecDeque;

use crate::TreeNode;

pub fn kth_smallest(root: TreeNode, k: i32) -> i32 {
    let mut list = vec![];
    let mut f = | x: i32 | {if !list.ends_with(&[x]) {
         list.push(x)};
    }  ;
    binary_search_recursive(&root, &mut f);
    list[k as usize - 1]
}

// steps:
// - finish `print_binary_search_order`
// - make it "higher order" (i.e. can do generic thing, not just printing)
// - use it to implement kth_smallest

// assume tree is in binary search order
// print nodes in ascending order

pub fn binary_search_recursive(root: &TreeNode, f: &mut impl FnMut(i32)) {
    if let Some(left) = &root.left {
        binary_search_recursive(left, f);
    }
    f(root.value);
    if let Some(right) = &root.right {
        binary_search_recursive(right, f);
    }
}

#[test]
fn feature() {
    let tree = TreeNode::new_binary_tree();
    assert_eq!(kth_smallest(tree, 6), 6)
}

fn binary_search_iter(root: &TreeNode, target: i32) {
    let mut current_node = root;

    loop {
        if current_node.value == target {
            println!("found");
            return;
        } else if target > current_node.value {
            if let Some(node) = &current_node.left {
                current_node = node;
            }
        } else if target < current_node.value {
            if let Some(node) = &current_node.right {
                current_node = node;
            }
        }
    }
}

fn binary_search_recur(root: &TreeNode, target: i32) {
    if root.value == target {
        println!("{}", root.value);
    } else if root.value > target {
        if let Some(left) = &root.left {
            binary_search_recur(left, target);
        }
    } else {
        if let Some(right) = root.right.as_deref() {
            binary_search_recur(right, target);
        }
    }
}

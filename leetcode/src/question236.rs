use crate::Tree;

// The root node is p or q. The answer cannot be below the root node, because then it would be missing the root (which is either p or q) as a descendant.
// One of p or q is in the left subtree, and the other one is in the right subtree. The root must be the answer because it is the connection point between the two subtrees, and thus the lowest node to have both p and q as descendants.
// Both p and q are in one of the subtrees. In that case, the root is not the answer because we could look inside the subtree and find a "lower" node.

pub fn lowest_common_ancestor(root: &Tree<i32>, p: i32, q: i32) -> &Tree<i32> {
    if p == root.value || q == root.value {
        return root;
    } 
    if in_left_side(root, p) && in_left_side(root, q) {
        return lowest_common_ancestor(root.left.as_ref().unwrap(), p, q);
    } else if !in_left_side(root, p) && !in_left_side(root, q) {
        return lowest_common_ancestor(root.right.as_ref().unwrap(), p, q);
    }
    root
}

pub fn in_left_side(tree: &Tree<i32>, target: i32) -> bool {
    let Some(left) = &tree.left else {
        return false;
    };
    tree_contains(left, target)
}

fn tree_contains(tree: &Tree<i32>, target: i32) -> bool {
    if tree.left.is_none() && tree.right.is_none() {
        return tree.value == target;
    }
    if tree.value == target {
        return true;
    }

    let is_in_left = if let Some(left) = &tree.left {
        tree_contains(left, target) 
    } else {
        false
    };
    if is_in_left {
        return true
    }

    let is_in_right = if let Some(right) = &tree.right {
        tree_contains(right, target) 
    } else {
        false
    };
    if is_in_right {
        return true
    }
    false       
}
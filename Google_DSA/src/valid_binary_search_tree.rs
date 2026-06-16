// Iterative in-order traversal

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub fn is_valid_bst(root: &TreeNode) -> bool {
    let mut min = 10000;
    let mut is_valid = true;

    let f = |x| {
        if x <= min {
            is_valid = false;
        }
        min = x;
    };
    is_valid_bst_iter(root, f);
    is_valid
}

pub fn is_valid_bst_iter(root: &TreeNode, mut f: impl FnMut(i32)) {
    let mut to_visit = vec![];
    let mut current = Some(root);
    while current.is_some() || !to_visit.is_empty() {
        // Go as far left as possible
        while let Some(node) = current {
            to_visit.push(node);
            current = node.left.as_deref();
        }

        // Visit node
        match to_visit.pop() {
            None => break,
            Some(node) => {
                f(node.val); // <-- visit happens HERE (in-order)
                current = node.right.as_deref();
            }
        }
    }
}

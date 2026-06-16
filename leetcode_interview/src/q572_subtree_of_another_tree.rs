use crate::TreeNode;

pub fn is_subtree(root: Option<Box<TreeNode>>, sub_root: Option<Box<TreeNode>>) -> bool {
    let mut to_visit = Vec::new();
    if root == sub_root {
        return true;
    }
    if let Some(root) = root.as_deref() {
        to_visit.push(root);
        while let Some(pop) = to_visit.pop() {
            if let Some(sub_root) = sub_root.as_deref() {
                if pop == sub_root {
                    return true;
                }
            }
            if let Some(left) = pop.left.as_deref() {
                to_visit.push(left);
            }
            if let Some(right) = pop.right.as_deref() {
                to_visit.push(right);
            }
        }
    }
    false
}
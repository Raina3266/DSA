use crate::TreeNode;

pub fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    let mut stack = vec![(p, q)];
    while let Some((node_p, node_q)) = stack.pop() {
        match (node_p, node_q) {
            (Some(n_p), Some(n_q)) => {
                if n_p.value != n_q.value {
                    return false;
                }
                 stack.push((n_p.left,n_q.left));
                 stack.push((n_p.right,n_q.right));
            },
            (None, None) => {
                continue;
            }
            _ => return false,
        }
    }
    true
}

use std::{cell::RefCell, rc::Rc};
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
// if both tree are empty, then return true.
// if tree is a leaf with the same value, then return true, else return false
// if the value of the p and q tree is the same, the left tree of p and q is the same, and the right tree of p and q is the same, then return true.
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    let (Some(p_node), Some(q_node)) = (p, q) else {
        return false;
    };
    let p_node = p_node.borrow();
    let q_node = q_node.borrow();

    if p_node.val != q_node.val {
        return false;
    }

    is_same_tree(p_node.left.clone(), q_node.left.clone()) && is_same_tree(p_node.right.clone(), q_node.right.clone())
}

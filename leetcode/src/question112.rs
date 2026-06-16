use crate::Tree;

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
use std::rc::Rc;
use std::cell::RefCell;


pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let Some(node) = root else {
        return false;
    };
    let node = node.borrow();

    if node.left.is_none() && node.right.is_none() {
        return target_sum == node.val;
    }

    let curr_sum = target_sum - node.val;
    if has_path_sum(node.left.clone(), curr_sum) || has_path_sum(node.right.clone(), curr_sum) {
        return true;
    }
    return false;
}



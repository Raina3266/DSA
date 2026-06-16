
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

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    let Some(node) = root else {
        return 0;
    };

    let node = node.borrow();

    if node.left.is_none() && node.right.is_none() {
        return 1;
    } 

    if node.left.is_none() && node.right.is_some() {
        return min_depth(node.right.clone()) + 1;
    } 

    if node.left.is_some() && node.right.is_none() {
        return min_depth(node.left.clone()) + 1;
    } 
    
    std::cmp::min(min_depth(node.left.clone()), min_depth(node.right.clone())) + 1
}
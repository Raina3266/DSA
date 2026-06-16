use std::cmp::max;

use crate::Tree;

pub fn _max_depth(root: Tree<i32>) -> i32 {
    let max_left = match root.left {
        Some(left) => _max_depth(*left),
        None => 0,
    };
    let max_right = match root.right {
        Some(right) => _max_depth(*right),
        None => 0,
    };
    max(max_left, max_right) + 1
}

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
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(tree) = root else {
            return 0;
        };

        let tree = tree.borrow();

        let left_max = Solution::max_depth(tree.left.as_ref().map(Rc::clone));
        let right_max = Solution::max_depth(tree.right.as_ref().map(Rc::clone));
        
        std::cmp::max(left_max, right_max) + 1
    }

}


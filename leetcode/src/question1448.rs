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
            right: None,
        }
    }
}

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(node) = root.clone() else {
        unreachable!();
    };
    let node = node.borrow();
    let max_ancestor = node.val;
    good_nodes_inner(root, max_ancestor)
}

pub fn good_nodes_inner(root: Option<Rc<RefCell<TreeNode>>>, mut max_ancestor: i32) -> i32 {
    let Some(node) = root else {
        return 0;
    };
    let node = node.borrow();
    let mut count = 0;
    if check_single_node(&node, max_ancestor) == true {
        count += 1;
    }
    max_ancestor = std::cmp::max(max_ancestor, node.val);
    count += good_nodes_inner(node.left.clone(), max_ancestor);
    count += good_nodes_inner(node.right.clone(), max_ancestor);
    count
}

pub fn check_single_node(root: &TreeNode, max_ancestor: i32) -> bool {
    root.val >= max_ancestor 
}

#[test]
fn refcell_explainer() {
    let x = Rc::new(RefCell::new(vec![1, 2, 3]));
    let y = x.clone();

    y.borrow_mut().push(4);
    println!("{x:?}");

    // let x = RefCell::new(123);  // RefCell is very much like a single-threaded mutex

    // let x_ref = x.borrow();      // Ref<_, T> is basically just a &T
    // let x_mut = x.borrow_mut();  // RefMut<_, T> is basically just a &mut T

    // println!("{x_ref}");
    // println!("{x_mut}");
}

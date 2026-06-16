use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}
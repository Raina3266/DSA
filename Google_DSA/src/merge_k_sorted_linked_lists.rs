// 1. to implement a PRIORITY QUEUE, the BINARY HEAP data structure is chosen
// why not vecdeque
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut queue = BinaryHeap::new();
    for list in lists {
        match list {
            Some(list) => {
                queue.push(list);
            }
            None => {}
        }
    }
    let mut dummy_node = Box::new(ListNode::new(0));
    let mut curr_node = &mut dummy_node;
    while let Some(node) = queue.pop() {
        let mut new_node = Box::new(ListNode::new(node.val));
        curr_node.next = Some(new_node);
        curr_node = curr_node.next.as_mut().unwrap();
        
        if node.next.is_some() {
            queue.push(node.next.unwrap());
        }
    }
    return dummy_node.next;
}

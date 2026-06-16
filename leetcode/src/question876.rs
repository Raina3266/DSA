use crate::ListNode;


pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast_pointer = head.as_ref();
    let mut slow_pointer = head.as_ref();
    while fast_pointer.is_some() && advance(fast_pointer).is_some(){
        slow_pointer = advance(slow_pointer);
        fast_pointer = advance(advance(fast_pointer));
    }
    return slow_pointer.cloned();
}

fn advance(node: Option<&Box<ListNode>>) -> Option<&Box<ListNode>> {
    node?.next.as_ref()
}
  
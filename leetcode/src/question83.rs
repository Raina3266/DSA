use crate::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pointer = head;
    // while let Some(curr) = pointer {
    //     if let Some(next_node) = curr.next {
    //         if curr.val == next_node.val{
    //             remove_next(pointer);

    //         }
    //     } 
    // }
todo!()
}

fn remove_next(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if node.is_none(){
        return None;
    }
    let mut node_inner = node.unwrap();
    if node_inner.next.is_none(){
        return Some(node_inner);
    }
    node_inner.next = node_inner.next.unwrap().next;
    Some(node_inner)
}
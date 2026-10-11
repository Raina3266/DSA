//! 23. Merge k Sorted Lists — https://leetcode.com/problems/merge-k-sorted-lists/
//!
//! You are given an array of `k` linked-lists `lists`, each linked-list is sorted
//! in ascending order.
//!
//! Merge all the linked-lists into one sorted linked-list and return it.
//!
//! Example 1:
//!   Input: lists = [[1,4,5],[1,3,4],[2,6]]
//!   Output: [1,1,2,3,4,4,5,6]
//!   Explanation: The linked-lists are:
//!   [
//!     1->4->5,
//!     1->3->4,
//!     2->6
//!   ]
//!   merging them into one sorted list:
//!   1->1->2->3->4->4->5->6
//!
//! Example 2:
//!   Input: lists = []
//!   Output: []
//!
//! Example 3:
//!   Input: lists = [[]]
//!   Output: []
//!
//! Constraints:
//!   * k == lists.length
//!   * 0 <= k <= 10^4
//!   * 0 <= lists[i].length <= 500
//!   * -10^4 <= lists[i][j] <= 10^4
//!   * lists[i] is sorted in ascending order.
//!   * The sum of lists[i].length will not exceed 10^4.

#![allow(dead_code, unused_variables)]

// Definition for singly-linked list (as provided by LeetCode).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in values.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = list {
            out.push(node.val);
            list = node.next;
        }
        out
    }

    #[test]
    fn example_1() {
        let lists = vec![
            to_list(vec![1, 4, 5]),
            to_list(vec![1, 3, 4]),
            to_list(vec![2, 6]),
        ];
        assert_eq!(to_vec(merge_k_lists(lists)), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn example_2_no_lists() {
        assert_eq!(to_vec(merge_k_lists(vec![])), Vec::<i32>::new());
    }

    #[test]
    fn example_3_single_empty_list() {
        assert_eq!(to_vec(merge_k_lists(vec![None])), Vec::<i32>::new());
    }

    #[test]
    fn single_list() {
        let lists = vec![to_list(vec![-1, 5, 7])];
        assert_eq!(to_vec(merge_k_lists(lists)), vec![-1, 5, 7]);
    }

    #[test]
    fn single_element_lists() {
        let lists = vec![to_list(vec![1]), to_list(vec![0])];
        assert_eq!(to_vec(merge_k_lists(lists)), vec![0, 1]);
    }
}

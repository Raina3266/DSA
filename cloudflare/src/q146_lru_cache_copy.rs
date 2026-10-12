//! 146. LRU Cache — https://leetcode.com/problems/lru-cache/
//!
//! Design a data structure that follows the constraints of a Least Recently Used
//! (LRU) cache.
//!
//! Implement the `LRUCache` class:
//!   * `LRUCache(capacity)` — Initialize the LRU cache with positive size `capacity`.
//!   * `get(key) -> i32` — Return the value of the `key` if the key exists,
//!     otherwise return -1.
//!   * `put(key, value)` — Update the value of the `key` if the key exists.
//!     Otherwise, add the key-value pair to the cache. If the number of keys
//!     exceeds the capacity from this operation, evict the least recently used key.
//!
//! The functions `get` and `put` must each run in O(1) average time complexity.
//!
//! Example:
//!   Input:
//!   ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
//!   [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
//!   Output: [null, null, null, 1, null, -1, null, -1, 3, 4]
//!   Explanation:
//!   LRUCache lRUCache = new LRUCache(2);
//!   lRUCache.put(1, 1); // cache is {1=1}
//!   lRUCache.put(2, 2); // cache is {1=1, 2=2}
//!   lRUCache.get(1);    // return 1
//!   lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
//!   lRUCache.get(2);    // returns -1 (not found)
//!   lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
//!   lRUCache.get(1);    // return -1 (not found)
//!   lRUCache.get(3);    // return 3
//!   lRUCache.get(4);    // return 4
//!
//! Constraints:
//!   * 1 <= capacity <= 3000
//!   * 0 <= key <= 10^4
//!   * 0 <= value <= 10^5
//!   * At most 2 * 10^5 calls will be made to get and put.

#![allow(dead_code, unused_variables)]

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type StrongLinkToNode = Rc<RefCell<Node>>;
type WeakLinkToNode = Weak<RefCell<Node>>;

pub enum Node {
    Data {
        key: i32,
        value: i32,
        prev: WeakLinkToNode,
        next: StrongLinkToNode,
    },
    Head(StrongLinkToNode),
    Tail(WeakLinkToNode),
}

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, StrongLinkToNode>,
    head: StrongLinkToNode,
    tail: StrongLinkToNode,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        let tail_node = Node::Tail(Weak::new());
        let tail = Rc::new(RefCell::new(tail_node));
        let head_node = Node::Head(tail.clone());
        let head = Rc::new(RefCell::new(head_node));

        let tail_node: &mut Node = &mut tail.borrow_mut();
        match tail_node {
            Node::Tail(weak) => *weak = Rc::downgrade(&head),
            _ => unreachable!(),
        };

        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            head,
            tail: tail.clone(),
        }
    }

    fn push_back(&self, input: &Rc<RefCell<Node>>) {
        let tail_node: &Node = &self.tail.borrow();

        let pointer_to_last_data_node = match tail_node {
            Node::Tail(last_data_node) => last_data_node.upgrade().unwrap(),
            _ => unreachable!(),
        };

        let last_data_node: &mut Node = &mut pointer_to_last_data_node.borrow_mut();

        // update old last data node
        match last_data_node {
            Node::Data { next, .. } => {
                *next = input.clone();
            }
            Node::Head(node) => {
                *node = input.clone();
            }
            _ => unreachable!(),
        };

        // update new last data node
        let input_node: &mut Node = &mut input.borrow_mut();
        match input_node {
            Node::Data { prev, next, .. } => {
                *next = self.tail.clone();
                *prev = Rc::downgrade(&pointer_to_last_data_node);
            }
            _ => unreachable!(),
        };

        // Update tail node
        let tail_node: &mut Node = &mut self.tail.borrow_mut();

        match tail_node {
            Node::Tail(weak) => *weak = Rc::downgrade(&input),
            _ => unreachable!(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        todo!()
    }
    pub fn put(&mut self, key: i32, val: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn get_missing_key_returns_minus_one() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(42), -1);
    }

    #[test]
    fn put_updates_existing_key_and_refreshes_recency() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10); // update key 1, making key 2 the LRU
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn get_refreshes_recency() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1); // key 1 is now most recently used
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(1), 1);
    }
}

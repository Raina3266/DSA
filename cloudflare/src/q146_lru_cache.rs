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

pub enum Node {
    Data {
        key: i32,
        value: i32,
        prev: Weak<RefCell<Node>>,
        next: Rc<RefCell<Node>>,
    },
    Head(Rc<RefCell<Node>>),
    Tail(Weak<RefCell<Node>>),
}

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        // Weak can be created empty (Weak::new()), but Rc cannot
        let tail_node = Node::Tail(Weak::new());
        let tail = Rc::new(RefCell::new(tail_node));
        let head_node = Node::Head(tail.clone());
        let head = Rc::new(RefCell::new(head_node));
        *tail.borrow_mut() = Node::Tail(Rc::downgrade(&head));

        LRUCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            head,
            tail,
        }
    }

    fn push_back(&self, input: Rc<RefCell<Node>>) {
        // 1. Upgrade the Weak inside Node::Tail to get the current last node
        // Read tail’s Weak, upgrade to Rc, no need of mutability
        let last: Rc<RefCell<Node>> = match &*self.tail.borrow() {
            Node::Tail(prev) => prev
                .upgrade()
                .expect("tail's back-pointer is never dangling"),
            _ => unreachable!("self.tail always holds Node::Tail"),
        };
        // 2. Change the next field of the last
        match &mut *last.borrow_mut() {
            Node::Data { next, .. } => {
                *next = input.clone();
            }
            // empty list
            Node::Head(next) => *next = input.clone(),
            _ => unreachable!(),
        }
        // 3. Change the prev and next field of the input
        match &mut *input.borrow_mut() {
            Node::Data {
                key,
                value,
                prev,
                next,
            } => {
                *prev = Rc::downgrade(&last);
                *next = self.tail.clone();
            }
            _ => unreachable!("push_back expects a data node"),
        }
        // 4. Change tail’s internal Weak to point at input
        *self.tail.borrow_mut() = Node::Tail(Rc::downgrade(&input));
    }

    fn move_to_end(&self, input: Rc<RefCell<Node>>) {
        // 1. Get the prev and next node of input
        let (prev_node, next_node) = match &*input.borrow() {
            Node::Data { prev, next, .. } => (
                prev.upgrade().expect("prev is never dangling"),
                next.clone(),
            ),
            _ => unreachable!("remove expects a data node"),
        };
        // 2. Change the next field of the prev node to input node
        match &mut *prev_node.borrow_mut() {
            Node::Data { next, .. } => {
                *next = next_node.clone();
            }
            Node::Head(next) => *next = next_node.clone(),
            _ => unreachable!("remove expects a data node"),
        }
        // 3. Change the prev field of the next node to input node
        match &mut *next_node.borrow_mut() {
            Node::Data { prev, .. } => {
                *prev = Rc::downgrade(&prev_node);
            }
            Node::Tail(prev) => *prev = Rc::downgrade(&prev_node),
            _ => unreachable!("remove expects a data node"),
        }
        self.push_back(input);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            let result = match &*node.borrow() {
                Node::Data { value, .. } => *value,
                _ => unreachable!("cache only stores data nodes"),
            };
            self.move_to_end(node.clone());
            result
        } else {
            -1
        }
    }

    fn is_full(&self) -> bool {
        self.cache.len() == self.capacity
    }

    fn pop_front(&self) -> Option<Rc<RefCell<Node>>> {
        let first = match &*self.head.borrow() {
            Node::Head(head_node) => head_node.clone(),
            _ => unreachable!(),
        };
        // empty list
        if !matches!(&*first.borrow(), Node::Data { .. }) {
            return None;
        }

        let second = match &*first.borrow() {
            Node::Data { next, .. } => next.clone(),
            _ => unreachable!(),
        };
        // list with only one node
        match &mut *second.borrow_mut() {
            Node::Data { prev, .. } => {
                *prev = Rc::downgrade(&self.head);
            }
            Node::Tail(prev) => *prev = Rc::downgrade(&self.head),
            _ => unreachable!(),
        }
        match &mut *self.head.borrow_mut() {
            Node::Head(head_node) => {
                *head_node = second;
            }
            _ => unreachable!(),
        }
        Some(first)
    }

    pub fn put(&mut self, key: i32, val: i32) {
        if let Some(node) = self.cache.get(&key) {
            match &mut *node.borrow_mut() {
                Node::Data { value, .. } => *value = val,
                _ => unreachable!(),
            }
            self.move_to_end(node.clone());
        } else {
            if self.is_full() {
                if let Some(evicted) = self.pop_front() {
                    let evict_key = match &*evicted.borrow() {
                        Node::Data { key, .. } => *key,
                        _ => unreachable!(),
                    };
                    self.cache.remove(&evict_key);
                };
            }
            let node = Rc::new(RefCell::new(Node::Data {
                key,
                value: val,
                prev: Weak::new(),
                next: self.tail.clone(),
            }));
            self.cache.insert(key, node.clone());
            self.push_back(node);
        }
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

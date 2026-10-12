//! 1381. Design a Stack With Increment Operation (finish)
//! https://leetcode.com/problems/design-a-stack-with-increment-operation/
//!
//! Design a stack that supports increment operations on its elements.
//!
//! Implement the `CustomStack` class:
//!   * `CustomStack(maxSize)` — Initializes the object with `maxSize` which is the
//!     maximum number of elements in the stack.
//!   * `push(x)` — Adds `x` to the top of the stack if the stack has not reached
//!     the `maxSize`.
//!   * `pop() -> i32` — Pops and returns the top of the stack or -1 if the stack
//!     is empty.
//!   * `increment(k, val)` — Increments the bottom `k` elements of the stack by
//!     `val`. If there are fewer than `k` elements in the stack, increment all the
//!     elements in the stack.
//!
//! Example 1:
//!   Input:
//!   ["CustomStack","push","push","pop","push","push","push","increment",
//!    "increment","pop","pop","pop","pop"]
//!   [[3],[1],[2],[],[2],[3],[4],[5,100],[2,100],[],[],[],[]]
//!   Output: [null,null,null,2,null,null,null,null,null,103,202,201,-1]
//!   Explanation:
//!   CustomStack stk = new CustomStack(3); // Stack is Empty []
//!   stk.push(1);                          // stack becomes [1]
//!   stk.push(2);                          // stack becomes [1, 2]
//!   stk.pop();                            // return 2 --> Return top of the stack 2, stack becomes [1]
//!   stk.push(2);                          // stack becomes [1, 2]
//!   stk.push(3);                          // stack becomes [1, 2, 3]
//!   stk.push(4);                          // stack still [1, 2, 3], Do not add another elements as size is 3
//!   stk.increment(5, 100);                // stack becomes [101, 102, 103]
//!   stk.increment(2, 100);                // stack becomes [201, 202, 103]
//!   stk.pop();                            // return 103 --> Return top of the stack 103, stack becomes [201, 202]
//!   stk.pop();                            // return 202 --> Return top of the stack 202, stack becomes [201]
//!   stk.pop();                            // return 201 --> Return top of the stack 201, stack becomes []
//!   stk.pop();                            // return -1 --> Stack is empty return -1.
//! 12345
//! 123
//! Constraints:
//!   * 1 <= maxSize, x, k <= 1000
//!   * 0 <= val <= 100
//!   * At most 1000 calls will be made to each method of increment, push and pop
//!     each separately.

#![allow(dead_code, unused_variables)]

pub struct CustomStack {
    stack: Vec<i32>,
    capacity: usize,
}

impl CustomStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(max_size),
            capacity: max_size,
        }
    }

    fn is_full(&self) -> bool {
        self.stack.len() == self.capacity
    }

    pub fn push(&mut self, x: i32) {
        if !self.is_full() {
            self.stack.push(x);
        }
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(pop) = self.stack.pop() {
            pop
        } else {
            -1
        }
    }

    pub fn increment(&mut self, k: i32, val: i32) {
        for element in self.stack.iter_mut().take(k as usize) {
            *element += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut stk = CustomStack::new(3);
        stk.push(1);
        stk.push(2);
        assert_eq!(stk.pop(), 2);
        stk.push(2);
        stk.push(3);
        stk.push(4); // ignored, stack is full
        stk.increment(5, 100);
        stk.increment(2, 100);
        assert_eq!(stk.pop(), 103);
        assert_eq!(stk.pop(), 202);
        assert_eq!(stk.pop(), 201);
        assert_eq!(stk.pop(), -1);
    }

    #[test]
    fn pop_on_empty_returns_minus_one() {
        let mut stk = CustomStack::new(3);
        assert_eq!(stk.pop(), -1);
    }

    #[test]
    fn push_beyond_max_size_is_ignored() {
        let mut stk = CustomStack::new(1);
        stk.push(1);
        stk.push(2); // ignored
        assert_eq!(stk.pop(), 1);
    }

    #[test]
    fn increment_with_k_larger_than_size_increments_all() {
        let mut stk = CustomStack::new(3);
        stk.push(1);
        stk.push(2);
        stk.increment(5, 10);
        assert_eq!(stk.pop(), 12);
        assert_eq!(stk.pop(), 11);
    }

    #[test]
    fn increment_only_bottom_k_elements() {
        let mut stk = CustomStack::new(3);
        stk.push(1);
        stk.push(2);
        stk.push(3);
        stk.increment(2, 5);
        assert_eq!(stk.pop(), 3); // top unchanged
        assert_eq!(stk.pop(), 7);
        assert_eq!(stk.pop(), 6);
    }
}

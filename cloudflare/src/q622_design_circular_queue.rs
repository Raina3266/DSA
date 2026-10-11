//! 622. Design Circular Queue — https://leetcode.com/problems/design-circular-queue/
//!
//! Design your implementation of the circular queue. The circular queue is a
//! linear data structure in which the operations are performed based on FIFO
//! (First In First Out) principle, and the last position is connected back to the
//! first position to make a circle. It is also called "Ring Buffer".
//!
//! One of the benefits of the circular queue is that we can make use of the
//! spaces in front of the queue. In a normal queue, once the queue becomes full,
//! we cannot insert the next element even if there is a space in front of the
//! queue. But using the circular queue, we can use the space to store new values.
//!
//! Implement the `MyCircularQueue` class:
//!   * `MyCircularQueue(k)` — Initializes the object with the size of the queue to be `k`.
//!   * `front() -> i32` — Gets the front item from the queue. If the queue is empty, return -1.
//!   * `rear() -> i32` — Gets the last item from the queue. If the queue is empty, return -1.
//!   * `enQueue(value) -> bool` — Inserts an element into the circular queue.
//!     Return `true` if the operation is successful.
//!   * `deQueue() -> bool` — Deletes an element from the circular queue.
//!     Return `true` if the operation is successful.
//!   * `isEmpty() -> bool` — Checks whether the circular queue is empty or not.
//!   * `isFull() -> bool` — Checks whether the circular queue is full or not.
//!
//! You must solve the problem without using the built-in queue data structure in
//! your programming language.
//!
//! Example 1:
//!   Input:
//!   ["MyCircularQueue", "enQueue", "enQueue", "enQueue", "enQueue", "Rear",
//!    "isFull", "deQueue", "enQueue", "Rear"]
//!   [[3], [1], [2], [3], [4], [], [], [], [4], []]
//!   Output: [null, true, true, true, false, 3, true, true, true, 4]
//!   Explanation:
//!   MyCircularQueue myCircularQueue = new MyCircularQueue(3);
//!   myCircularQueue.enQueue(1); // return True
//!   myCircularQueue.enQueue(2); // return True
//!   myCircularQueue.enQueue(3); // return True
//!   myCircularQueue.enQueue(4); // return False
//!   myCircularQueue.Rear();     // return 3
//!   myCircularQueue.isFull();   // return True
//!   myCircularQueue.deQueue();  // return True
//!   myCircularQueue.enQueue(4); // return True
//!   myCircularQueue.Rear();     // return 4
//!
//! Constraints:
//!   * 1 <= k <= 1000
//!   * 0 <= value <= 1000
//!   * At most 3000 calls will be made to enQueue, deQueue, front, rear, isEmpty,
//!     and isFull.

#![allow(dead_code, unused_variables)]

pub struct MyCircularQueue {}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        todo!()
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        todo!()
    }

    pub fn de_queue(&mut self) -> bool {
        todo!()
    }

    pub fn front(&self) -> i32 {
        todo!()
    }

    pub fn rear(&self) -> i32 {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn is_full(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(!q.en_queue(4)); // full
        assert_eq!(q.rear(), 3);
        assert!(q.is_full());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(q.rear(), 4);
    }

    #[test]
    fn empty_queue_front_and_rear_are_minus_one() {
        let q = MyCircularQueue::new(3);
        assert!(q.is_empty());
        assert_eq!(q.front(), -1);
        assert_eq!(q.rear(), -1);
    }

    #[test]
    fn dequeue_from_empty_returns_false() {
        let mut q = MyCircularQueue::new(2);
        assert!(!q.de_queue());
    }

    #[test]
    fn freed_space_can_be_reused() {
        let mut q = MyCircularQueue::new(2);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.de_queue());
        assert!(q.en_queue(3));
        assert_eq!(q.front(), 2);
        assert_eq!(q.rear(), 3);
        assert!(!q.en_queue(4)); // full again
    }

    #[test]
    fn capacity_one() {
        let mut q = MyCircularQueue::new(1);
        assert!(q.en_queue(5));
        assert!(q.is_full());
        assert!(!q.en_queue(6));
        assert_eq!(q.front(), 5);
        assert_eq!(q.rear(), 5);
        assert!(q.de_queue());
        assert!(q.is_empty());
        assert!(q.en_queue(7));
        assert_eq!(q.front(), 7);
    }
}

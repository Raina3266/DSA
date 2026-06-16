use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone)]
struct KthLargest {
    k: i32,
    nums: Vec<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}
#[allow(clippy::collapsible_else_if)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums.clone());
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        if k <= nums.len().try_into().unwrap() {
            for _ in 0..k {
                let num = heap.pop().unwrap();
                min_heap.push(Reverse(num));
            }
        } else {
            if !nums.is_empty() {
                for _ in 0..nums.len() {
                    let num = heap.pop().unwrap();
                    min_heap.push(Reverse(num));
                }
            }
        }
        KthLargest { k, nums, min_heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.k as usize {
            self.min_heap.push(Reverse(val));
        } else if val > self.min_heap.peek().unwrap().0 {
            self.min_heap.push(Reverse(val));
            self.min_heap.pop().unwrap();
        }

        self.min_heap.peek().unwrap().0
    }
}

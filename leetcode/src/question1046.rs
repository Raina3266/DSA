use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
    while heap.len() > 1 {
        let num = heap.pop().unwrap() - heap.pop().unwrap();
        if num != 0 {
            heap.push(num);
        }
    }
    heap.pop().unwrap_or(0)
}
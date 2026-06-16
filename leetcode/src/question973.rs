use std::{cmp::Reverse, collections::BinaryHeap};

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut heap: BinaryHeap<Reverse<(i32, Vec<i32>)>> = BinaryHeap::new();
    for pair in points {
        let distance = pair[0] * pair[0] + pair[1] * pair[1];
        heap.push(Reverse((distance, pair)));
    }
    for _ in 0..k as usize {
        let max = heap.pop().unwrap().0;
        ans.push(max.1);
    }
    ans
}
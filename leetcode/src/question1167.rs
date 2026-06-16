use std::{cmp::Reverse, collections::BinaryHeap};

pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut min_heap: BinaryHeap<Reverse<i32>> = sticks.into_iter().map(Reverse).collect();
    let mut cost = 0;

    while min_heap.len() > 1 {
        let x = min_heap.pop().unwrap();
        let y = min_heap.pop().unwrap();
        let new = x.0 + y.0;
        cost += new;
        min_heap.push(Reverse(new));
    }
    cost
}

use std::{cmp::Reverse, collections::BinaryHeap};

pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = weight.iter().map(|a| Reverse(*a)).collect();
    let mut sum = 0;
    let mut ans = 0;

    while !heap.is_empty() {
        let x = heap.pop().unwrap().0;
        sum += x;
        if sum > 5000 {
            return ans;
        }
        ans += 1
    }

    ans
}

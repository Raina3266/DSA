use std::collections::BinaryHeap;

pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut count = k;
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(piles);
    while count != 0 {
        let max = heap.pop().unwrap();
        let mut remain = 0;
        if max % 2 == 0 {
            remain = max / 2;
        } else {
            remain = max / 2 + 1;
        }
        heap.push(remain);
        count -= 1;
    }
    heap.iter().sum()
}
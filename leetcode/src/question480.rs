use std::{
    cmp::Reverse,
    collections::BinaryHeap,
};

pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let mut ans = vec![];
    let usize_k: usize = k.try_into().unwrap();
    let window = &nums[0..usize_k];
    // Initialization of Heaps
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(window.to_vec());
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for _ in 0..(k / 2) as usize {
        let n = max_heap.pop().unwrap();
        min_heap.push(Reverse(n));
    }
    let f = calculate_the_median(&max_heap, &min_heap);
    ans.push(f);

    for i in 0..nums.len() - usize_k {
        // Removing element nums[i]
        if nums[i] <= *max_heap.peek().unwrap() {
            if nums[i] == *max_heap.peek().unwrap() {
                max_heap.pop().unwrap();
            } else {
                remove_one_element_max(&mut max_heap, nums[i]);
            }
        } else {
            remove_one_element_min(&mut min_heap, nums[i]);
        }
        // adding element nums[i + usize_k]
        if nums[i + usize_k] <= *max_heap.peek().unwrap() {
            max_heap.push(nums[i + usize_k]);
        } else {
            min_heap.push(Reverse(nums[i + usize_k]));
        }
        // rebalance two heaps
        balance_two_heaps(&mut max_heap, &mut min_heap);
        let f = calculate_the_median(&max_heap, &min_heap);
        ans.push(f);
    }
    ans
}

fn balance_two_heaps(max_heap: &mut BinaryHeap<i32>, min_heap: &mut BinaryHeap<Reverse<i32>>) {
    while min_heap.len() > max_heap.len() {
        max_heap.push(min_heap.pop().unwrap().0);
    }
    while max_heap.len() - min_heap.len() > 1 {
        min_heap.push(Reverse(max_heap.pop().unwrap()))
    }
}

fn remove_one_element_max(heap: &mut BinaryHeap<i32>, remove: i32) {
    let mut has_remove = false;
    heap.retain(|i| {
        if has_remove {
            return true;
        }
        if *i == remove {
            has_remove = true;
            return false;
        }
        true
    });
}

fn remove_one_element_min(heap: &mut BinaryHeap<Reverse<i32>>, remove: i32) {
    let mut has_remove = false;
    heap.retain(|i| {
        if has_remove {
            return true;
        }
        if i.0 == remove {
            has_remove = true;
            return false;
        }
        true
    });
}


fn calculate_the_median(max_heap: &BinaryHeap<i32>, min_heap: &BinaryHeap<Reverse<i32>>) -> f64 {
    let mut f: f64 = 0.0;
    if max_heap.len() > min_heap.len() {
        f = *max_heap.peek().unwrap() as f64;
    }
    if max_heap.len() == min_heap.len() {
        let x = *max_heap.peek().unwrap() as f64;
        let y = min_heap.peek().unwrap().0 as f64;
        f = (x + y) / 2.0;
    }
    (f * 100000.0).round() / 100000.0
}

// use std::{cmp::Reverse, collections::BinaryHeap};

// pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
//     let mut ans = vec![];
//     let usize_k: usize = k.try_into().unwrap();
//     let window = &nums[0..usize_k];
//     let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(window.to_vec());
//     let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

//     let f = balance_two_heaps(&mut max_heap, &mut min_heap);
//     ans.push(f);

//     for i in 0..nums.len() - usize_k {
//         max_heap.push(nums[i + usize_k]);
//         let max_length = max_heap.len();
//         max_heap.retain(|a| *a != nums[i]);
//         min_heap.retain(|a| a.0 != nums[i]);
//         if max_heap.len() != max_length && nums[i + usize_k] == nums[i] {
//             max_heap.push(nums[i])
//         }
//         let f = balance_two_heaps(&mut max_heap, &mut min_heap);
//         ans.push(f);
//     }
//     ans
// }

// fn balance_two_heaps(
//     max_heap: &mut BinaryHeap<i32>,
//     min_heap: &mut BinaryHeap<Reverse<i32>>,
// ) -> f64 {
//     loop {
//         min_heap.push(Reverse(max_heap.pop().unwrap()));
//         if min_heap.len() > max_heap.len() {
//             max_heap.push(min_heap.pop().unwrap().0);
//             if max_heap.len() > min_heap.len() {
//                 let f = *max_heap.peek().unwrap() as f64;
//                 return (f * 100000.0).round() / 100000.0;
//             }
//             if max_heap.len() == min_heap.len() {
//                 let x = *max_heap.peek().unwrap() as f64;
//                 let y = min_heap.peek().unwrap().0 as f64;
//                 let f = (x + y) / 2.0;
//                 return (f * 100000.0).round() / 100000.0;
//             }
//         }
//     }
// }

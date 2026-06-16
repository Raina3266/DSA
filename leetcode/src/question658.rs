use std::collections::BinaryHeap;

#[allow(clippy::collapsible_else_if)]
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    if x <= arr[0] {
        arr.into_iter().take(k as usize).collect()
    } else if x >= arr[arr.len() - 1] {
        let mut new_arr = arr;
        new_arr.reverse();
        return new_arr.into_iter().take(k as usize).rev().collect();
    } else {
        let mut ans: Vec<i32> = find_closest_elements_middle(arr, k, x);
        ans.sort();
        return ans
    }
}

#[test]
fn feature() {
    let vec = vec![1, 1, 1, 10, 10, 10];
    assert_eq!(find_closest_elements(vec, 1, 9), vec![10])
}

fn find_closest_elements_middle(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut vec: Vec<(i32, i32)> = vec![];
    for i in arr {
        vec.push(((i-x).abs(),i));
    }
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::from(vec);
    while heap.len() > (k as usize) {
        heap.pop().unwrap();
    }
    let ans: Vec<i32> = heap.into_iter().map(|(_, b)| b).collect();
    ans
}

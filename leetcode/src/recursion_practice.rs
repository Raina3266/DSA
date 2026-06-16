// step 1: identify the base case
// step 2: shrink/split the input
// step 3: calculation the smaller inout and assume the smaller input just works
// step 4: merge the smaller input with the rest of the data


use std::vec;

#[allow(clippy::needless_bool)]
fn is_even(x: u64) -> bool {
    if x == 0 {
        return true;
    }

    if is_even(x - 1) {
        false
    } else {
        true
    }
}

fn list_length(list: &[i32]) -> usize {
    if list.is_empty() {
        return 0;
    }
    list_length(&list[1..]) + 1
}

fn list_max(list: &[i32]) -> i32 {
    if list.is_empty() {
        i32::MIN
    } else {
        let (head, tail) = list.split_first().unwrap();
        std::cmp::max(*head, list_max(tail))
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn binary_search(sorted_list: &[i32], target: i32) -> usize {
    if sorted_list.len() == 1 {
        return 0;
    }
    let midpoint = sorted_list.len() / 2;

    let smaller = &sorted_list[0..midpoint];
    let bigger = &sorted_list[midpoint..];
    if target >= sorted_list[midpoint] {
        midpoint + binary_search(bigger, target)
    } else {
        binary_search(smaller, target)
    }
}

#[test]
fn feature() {
    let sorted_list = &[1,2,3,4,7,8,13486,213867,213768,1386074];
    let target = 8;
    assert_eq!(binary_search(sorted_list, target),5);
}

// merge sort is an algorithm for sorting a list
// you split the list in half, sort each half, then merge them together in the correct order

fn sort(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    } else if nums.len() ==  1 {
        return nums;
    }
    let mid = nums.len() / 2;
    let (left, right) = nums.split_at(mid);
    merge_two_lists(sort(left.to_vec()), sort(right.to_vec()))
}

fn merge_two_lists(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut pointer1 = 0;
    let mut pointer2 = 0;
    while pointer1 < nums1.len() && pointer2 < nums2.len() {
        if nums1[pointer1] >= nums2[pointer2] {
            ans.push(nums2[pointer2]);
            pointer2 += 1;
        } else {
            ans.push(nums1[pointer1]);
            pointer1 += 1;
        }
    }
    if pointer1 == nums1.len() {
        ans.extend(nums2[pointer2..].to_vec());
    }
    if pointer2 == nums2.len() {
        ans.extend(nums1[pointer1..].to_vec());
    }
    ans
}

#[test]
fn merge() {
    assert_eq!(sort(vec![999, 1, 3, 2, 4, 0, 0, 0]), vec![0, 0, 0, 1, 2, 3, 4, 999]);
}

#[test]
fn merge_two() {
    assert_eq!(merge_two_lists(vec![1,2,3,4], vec![3,4,8,10]), vec![1,2,3,3,4,4,8,10]);
}
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

use std::collections::HashMap;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut nums =nums;
    nums.sort(); 

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue; // Skip duplicates
        }
        let target = -nums[i];
        let other_two = two_sum_i(&nums[i + 1..], target);
        for mut pair in other_two {
            pair.push(nums[i]);
            ans.push(pair);
        }
    }
    ans.dedup();
    ans
}

pub fn two_sum_i(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut new_nums = vec![];
    for (i, n) in nums.iter().enumerate() {
        let needed_num = target - n;

        if map.contains_key(n) {
            let mut one_pair = vec![];
            let index = map.get(n).unwrap();
            one_pair.push(*n);
            one_pair.push(nums[*index]);
            new_nums.push(one_pair);
        }
        
        map.insert(needed_num, i);
    }
    new_nums
}

#[test]
fn test() {
    let vec = vec![1,2,-2,-1];
    assert_eq!(three_sum(vec), vec![vec![]]);
}
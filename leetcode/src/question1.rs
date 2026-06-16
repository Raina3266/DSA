// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// Example:
// fn two_sum_worse(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     todo!()
// }

//slice: &[i32](pointer) is cheaper than Vec<i32>(string)

use std::collections::HashMap;



pub fn two_sum(nums: &[i32], target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        if nums[left] + nums[right] > target {
            right -= 1;
        } else if nums[left] + nums[right] < target {
            left += 1;
        } else if nums[left] + nums[right] == target {
            return true;
        }
    }
    false
}

pub fn two_sum_in_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (index, num) in nums.into_iter().enumerate() {
        let other_num = target - num;

        if map.contains_key(&other_num) {
            return vec![index as i32, map[&other_num] as i32];
        } 

        map.insert(num, index);
    }
    vec![]
}


#[test]
fn example_1() {
    let nums = [2, 7, 11, 15];
    let target = 20;

    let x = two_sum(&nums, target);

    assert_eq!(x, false);
}

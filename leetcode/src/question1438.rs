// max - min <= limit


pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    use std::{cmp::max, collections::VecDeque};

    let mut ans = 1;
    let mut increasing: VecDeque<i32> = VecDeque::new();
    increasing.push_back(nums[0]);
    let mut decreasing: VecDeque<i32> = VecDeque::new();
    decreasing.push_back(nums[0]);
    let mut left = 0;

    for i in 1..nums.len() {
        while !increasing.is_empty() && &nums[i] <= increasing.back().unwrap(){
            increasing.pop_back().unwrap();
        }
        increasing.push_back(nums[i]);
        while !decreasing.is_empty() && &nums[i] >= decreasing.back().unwrap(){
            decreasing.pop_back().unwrap();
        }
        decreasing.push_back(nums[i]);
        
        while !decreasing.is_empty() && !increasing.is_empty() && decreasing[0] - increasing[0] > limit {
            if &nums[left] == increasing.front().unwrap(){
                increasing.pop_front().unwrap();
            }
            if &nums[left] == decreasing.front().unwrap(){
                decreasing.pop_front().unwrap();
            }
            left += 1; 
        }
        ans = max(ans, i - left + 1);
    }
    ans as i32
}
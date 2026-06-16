    
pub fn rob(nums: Vec<i32>) -> i32 {
    // dp to store the maximum amount of money that can be robbed up to the current house.
    let mut dp: Vec<i32> = vec![0; nums.len()];

    if dp.len() == 1 {
        return nums[0];
    }
    if dp.len() == 2 {
        return std::cmp::max(nums[0], nums[1]);
    }
    
    dp[0] = nums[0];
    dp[1] = std::cmp::max(nums[0], nums[1]);

    for i in 2..nums.len() {
        dp[i] = std::cmp::max(nums[nums.len() - 1] + nums[nums.len() - 3], nums[nums.len() - 2]);
    }
    
    dp.pop().unwrap()
}


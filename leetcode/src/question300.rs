// nums = [10,9,2,5,3,7,101,18]

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];

    if nums.len() == 1 {
        return 1;
    }

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1
            }
        }
    }
    *dp.iter().max().unwrap()
}

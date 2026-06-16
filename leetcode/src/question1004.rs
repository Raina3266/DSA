

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut num_zero = 0;
    let mut start = 0;
    let mut end = 0;
    let mut answer = 0;
        
    while end < nums.len() && start < nums.len() {
        if nums[end] == 1 {
            end += 1;
        } else if nums[end] == 0 && num_zero < k {
            end += 1;
            num_zero += 1;
        } else if nums[start] == 0 {
            start += 1;
            num_zero -= 1;
        } else {
            start += 1;
        }
        answer = std::cmp::max(answer, (end - start) as i32)
    }
    return answer;
}

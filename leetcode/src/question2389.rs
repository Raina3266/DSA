pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();
    let mut ans = vec![];
    for q in queries {
        ans.push(check_one_query(&nums, q));
    }
    ans
}

fn check_one_query(nums: &[i32], query: i32) -> i32 {
    let mut sum = 0;
    for (i, n) in nums.iter().enumerate() {
        sum += n;
        if sum > query {
            return i  as i32;
        }
    }
    nums.len().try_into().unwrap()
}
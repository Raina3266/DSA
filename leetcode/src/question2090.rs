
pub fn get_averages(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let i64_nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut prefix_sum = vec![];
    prefix_sum.push(i64_nums[0]);

    for i in 1..=i64_nums.len() -1 {
        prefix_sum.push(i64_nums[i] + prefix_sum[prefix_sum.len() - 1]);
    }

    let mut ans: Vec<i64> = vec![];
    let n: i64 = (2*k + 1).try_into().unwrap();

    for x in 0..=i64_nums.len() -1 {
       if x < k || x > i64_nums.len() - k - 1 {
            ans.push(-1);
       } else {
            let y = prefix_sum[x + k] - prefix_sum[x - k] + i64_nums[x - k];
            ans.push(y/n);
       }
    }
    ans.into_iter().map(|x| x as i32).collect()
}
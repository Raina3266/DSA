
pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let i64_nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut prefix_sum = vec![];
    prefix_sum.push(i64_nums[0]);

    for i in 1..=i64_nums.len() -1 {
        prefix_sum.push(i64_nums[i] + prefix_sum[prefix_sum.len() - 1]);
    }

    let mut count = 0;

    for q in 0..prefix_sum.len() -1 {
        let second_half = prefix_sum[prefix_sum.len() - 1] - prefix_sum[q];
        if prefix_sum[q] >= second_half {
            count += 1;
        }
    }
    count
}
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut prefix_sum: Vec<i32> = vec![0; nums.len()];
    let mut count = 0;
    prefix_sum[0] = nums[0];
    map.insert(nums[0], 1);

    for n in 0..nums.len() {
        prefix_sum[n] = prefix_sum[n - 1] + nums[n];
        if prefix_sum[n] == k {
            count += 1;
        }
        count += *map.get(&(prefix_sum[n] - k)).unwrap_or(&0);
        *map.entry(prefix_sum[n]).or_insert(0) += 1;
    }
    count
}
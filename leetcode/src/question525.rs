

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let mut prefix_sum = 0;
    let mut ans = 0;

    for (index, num) in nums.into_iter().enumerate() {
        prefix_sum += if num == 1 { 1 } else { -1 };

        if map.contains_key(&prefix_sum) {
            ans = max(ans, index as i32 - map.get(&prefix_sum).unwrap());
        } else {
            map.insert(prefix_sum, index as i32);
        }
    }
    ans
}

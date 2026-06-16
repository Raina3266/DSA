use std::collections::HashSet;
// this is still overall o(n), no need to revisit
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut count = 1;
    let set: HashSet<i32> = nums.into_iter().collect();
    for num in set.iter() {
        if !set.contains(&(num - 1)) {
            let mut num = num + 1;
            let mut inner_count = 1;
            while set.contains(&num) {
                num += 1;
                inner_count += 1;
            }
            count = count.max(inner_count);
        }
    }
    count
}

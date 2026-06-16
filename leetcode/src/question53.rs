// Given an integer array nums, find the subarray with the largest sum, and return its sum.

pub fn max_sub_array(nums: &[i32]) -> i32 {
    let mut current_prefix_sum = 0;
    let mut max_sum = i32::MIN;
    let mut min_prefix_sum = 0;
    for n in nums {
        current_prefix_sum += n;
        max_sum = max_sum.max(current_prefix_sum - min_prefix_sum);
        min_prefix_sum = min_prefix_sum.min(current_prefix_sum );
    }
    max_sum
}

#[test]
fn feature() {
    let input = vec![-2,-1];
    assert_eq!(max_sub_array(&input), -1);
}

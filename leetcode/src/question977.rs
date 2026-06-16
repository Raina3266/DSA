use test_strategy::proptest;

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut sorted = Vec::new();

    if nums.is_empty(){
        return nums;
    }
    if nums.len() == 1 {
        return vec![nums[0] * nums[0]];
    }

    while left <= right {
        if nums[left] * nums[left] >= nums[right] * nums[right] {
            sorted.push(nums[left] * nums[left]);
            left += 1;
        } else {
            sorted.push(nums[right] * nums[right]);
            right -= 1;
        }
    }
    sorted.reverse();
    sorted
}

#[test]
fn overflow() {
    sorted_squares(vec![0]);
}
#[proptest]
fn feature(#[strategy(proptest::collection::vec(-10000..=10000, 1..1000))] mut nums: Vec<i32>) {

    nums.sort();

    let result = sorted_squares(nums);

    let mut result_sorted = result.clone();
    result_sorted.sort();

    assert_eq!(result, result_sorted);
}

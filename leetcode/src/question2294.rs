pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut ans = 0;
    loop {
        let max = nums[0] + k;
        let index = return_index(&nums, max);
        ans += 1;
        nums.drain(..=index);
        if nums.is_empty() {
            return ans;
        }
    }
}

fn return_index(nums: &[i32], max: i32) -> usize {
    let mut index = 0; 
    for n in nums {
        if *n <= max {
            index += 1;
        } else {
            return index - 1
        }
    }
    index - 1
}

#[test]
fn feature() {
    let nums = &[4,5];
    assert_eq!(return_index(nums, 4), 0);
}
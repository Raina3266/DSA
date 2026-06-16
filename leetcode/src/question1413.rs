
pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut start_value = 1;

    loop {
        let mut sum = start_value;
        let mut new_vec: Vec<_> = vec![];
        for index in 0..nums.len() {
            sum += nums[index];
            new_vec.push(sum);
        }

        if new_vec.iter().all(|n| *n > 0) {
            return start_value;
        } 
        start_value += 1;
    }

}

#[test]
fn tesst() {
    let nums = vec![1,-2,-3];
    assert_eq!(min_start_value(nums), 5);
}

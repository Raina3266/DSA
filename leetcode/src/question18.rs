pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    if nums.len() < 4 {
        return result;
    }
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {

        for n in (i + 1)..nums.len() {

            let mut left = n + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] as i64 + nums[n] as i64 + nums[left] as i64 + nums[right] as i64;
                if sum < target as i64 {
                    left += 1;
                } else if sum > target as i64 {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[n], nums[left], nums[right]]);
                    left += 1;
                }
            }
        }
    }   
    result.dedup();
    result
}

#[test]
fn feature() {
    let nums = vec![2,2,2,2,2];
    let target = 8;
    assert_eq!(four_sum(nums, target), vec![vec![2,2,2,2]]);
}

// println!("{i}");
// println!("{n}");
// println!("{left}");
// println!("{right}");
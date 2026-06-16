use std::collections::HashMap;
// do the better solution again
pub fn three_sum_better(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut nums = nums;
    nums.sort();
    for n1 in 0..nums.len() {
        if n1 > 0 && nums[n1] == nums[n1 - 1] {
            continue;
        }
        let mut left = n1 + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[n1] + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                result.push(vec![nums[n1], nums[left], nums[right]]);
                left += 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }
    result
}

#[test]
fn feature() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(three_sum_better(nums), output);
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut nums = nums;
    nums.sort();
    let mut map: HashMap<i32, usize> = HashMap::new();
    for n1 in 0..nums.len() {
        for n2 in (n1 + 1)..nums.len() {
            let mut pair = vec![];
            let target = 0 - (nums[n1] + nums[n2]);
            if let Some(n3) = map.get(&target) {
                pair.push(nums[n1]);
                pair.push(nums[n2]);
                pair.push(nums[*n3]);
                if result.contains(&pair) {
                    result.push(pair);
                }
            }
        }
        map.insert(nums[n1], n1);
    }
    result
}

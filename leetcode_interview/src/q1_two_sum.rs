use std::collections::HashMap;

// do it as O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let remain = target - num;
        if let Some(index2) = map.get(&remain) {
            result.push(*index2 as i32);
            result.push(index as i32);
            return result;
        }
        map.insert(*num, index);
    }
    result
}

#[test]
fn feature() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let answer = vec![0,1];
    assert_eq!(two_sum(nums, target), answer);
}
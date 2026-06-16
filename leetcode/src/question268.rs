use std::collections::HashSet;

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut sett: HashSet<usize> = (0..=n).into_iter().collect();

    for x in nums {
        sett.remove(&(x as usize));

    }
    sett.iter().next().cloned().unwrap().try_into().unwrap()
}
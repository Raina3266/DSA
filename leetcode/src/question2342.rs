use std::{cmp::{max, Reverse}, collections::HashMap};

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for n in nums {
        let mut num = n;
        let mut digit: Vec<i32> = vec![];
        while num > 0 {
            digit.push(num % 10);
            num /= 10;
        }
        let sum: i32 = digit.into_iter().sum();
        map.entry(sum).and_modify(|numbers_with_same_sum| numbers_with_same_sum.push(n)).or_insert(vec![n]);
    }
    if map.values().all(|v| v.len() == 1){
        return -1;
    }
    for m in map.iter(){
        let mut nums = m.1.clone();
        if nums.len() > 1 {
            nums.sort_by_key(|&x| Reverse(x));
            ans = max(ans, nums[0] + nums[1]);
        }
    }
    ans
}
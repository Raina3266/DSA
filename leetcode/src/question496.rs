use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, n) in nums2.iter().enumerate(){
        map.insert(*n, i as i32);
    }
    for nn in nums1{
        let position = map.get(&nn).unwrap();
        let x = map.iter().filter(|(key, value)| **key > nn && *value > position).map(|(_, v)| v).min().unwrap_or(&-1);
        if *x == -1 {
            ans.push(-1);
        } else {
            ans.push(nums2[*x as usize]);
        }
    }
    ans
}
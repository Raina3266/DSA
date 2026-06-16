use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut vec: Vec<(i32, usize)> = vec![];
    for i in nums {
        *map.entry(i).or_insert(0) += 1
    }
    map.into_iter().for_each(|(k, v)| vec.push((k, v)));
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    for i in vec.into_iter().take(k as usize) {
        ans.push(i.0);
    }
    ans
}
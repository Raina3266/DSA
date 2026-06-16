use std::collections::HashMap;

pub fn min_set_size(arr: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut length: i32 = arr.len().try_into().unwrap();
    let half = length / 2;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in arr {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut all_values: Vec<&i32> = map.values().clone().collect();
    all_values.sort_by(|a, b| b.cmp(a));
    for ii in all_values {
        length -= *ii;
        ans += 1;
        if length < half {
            return ans;
        }
    }
    ans
}

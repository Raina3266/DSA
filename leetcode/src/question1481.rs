use std::collections::HashMap;

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut k = k;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in arr {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut all_values: Vec<&i32> = map.values().clone().collect();
    all_values.sort();
    for a in all_values {
        k -= a;
        if k < 0 {
            ans += 1;
        }
    }
    ans
}
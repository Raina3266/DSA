use std::collections::HashSet;

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let set: HashSet<_> = arr.iter().cloned().collect();
    let mut count = 0;

    for n in arr {
        if set.contains(&(n+1)) {
            count += 1;
        }
    }
    count
}
use std::collections::HashSet;

// finish with the best solution within 5 mins, no need to revisit

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for n in nums {
        // a better way
        if !set.insert(n) {
            return true;
        }
    }
    false
}

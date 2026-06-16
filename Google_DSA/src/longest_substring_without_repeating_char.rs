use std::{cmp::max, collections::HashSet};

pub fn length_of_longest_substring(s: String) -> i32 {
    let all_chars: Vec<char> = s.chars().collect();
    let mut chars = HashSet::new();
    let mut length = 0;
    let mut left = 0usize;
    let mut right = 0usize;
    for right in 0..all_chars.len() {
        while chars.contains(&all_chars[right]) {
            chars.remove(&all_chars[left]);
            left += 1;
        }
        chars.insert(all_chars[right]);

        length = max(length, right - left + 1);

    }
    length as i32
}

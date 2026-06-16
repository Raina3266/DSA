// Given a string s, find the length of the longest substring without repeating characters.
use std::collections::HashSet;


pub fn length_of_longest_substring(s: &str) -> usize {
    let s_list = s.as_bytes();
    let mut sett = HashSet::new();

    let mut left = 0;
    let mut right = 0;
    let mut answer = 0;
    while left < s_list.len() && right < s_list.len() {
       if sett.insert(s_list[right]) {
          right += 1;
       } else {
          sett.remove(&s_list[left]);
          left += 1
       }
       
       answer = std::cmp::max(answer, right - left)
    }
    answer
}


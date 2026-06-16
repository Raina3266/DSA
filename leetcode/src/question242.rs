// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

use std::collections::HashMap;

pub fn is_anagram(s: &str, t: &str) -> bool { 
   calculate_char_counts(s) == calculate_char_counts(t)
}

fn calculate_char_counts(string: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();    
    for s in string.chars() {
        *map.entry(s).or_insert(0) += 1;
    }
    map
}
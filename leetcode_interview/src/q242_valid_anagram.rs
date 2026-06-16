use std::collections::HashMap;
//  no need to revisit
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_letters = HashMap::new();
    let mut t_letters = HashMap::new();
    s.chars().for_each(|char| {
        *s_letters.entry(char).or_insert(0) += 1;
    });
    t.chars().for_each(|char| {
        *t_letters.entry(char).or_insert(0) += 1;
    });
    s_letters == t_letters
}

// If the problem guarantees only lowercase English letters (a-z)
// you can do even better in practice using a fixed-size array (hashmap is slow)


pub fn is_anagram_better(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = [0; 26];

    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    for b in t.bytes() {
        counts[(b - b'a') as usize] -= 1;
    }

    counts.iter().all(|&x| x == 0)
}
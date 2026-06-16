use std::collections::HashMap;

pub fn are_occurrences_equal(s: String) -> bool {
    let list:Vec<char> = s.chars().collect();
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map.iter().all(|(_, value)| value == map.get(&list[0]).unwrap())
}
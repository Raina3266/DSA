use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<char>> = HashMap::new();
    for s in strs {
        let v: Vec<char> = s.chars().collect();
        map.insert(s, v);
    }
    todo!()
}


pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut sorted_string: Vec<char> = s.clone().chars().collect();
        sorted_string.sort();
        map.entry(sorted_string).and_modify(|v| v.push(s.clone())).or_insert(vec![s.clone()]);
    }

    let ans: Vec<Vec<String>> = map.values().cloned().collect();
    ans
}
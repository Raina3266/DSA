pub fn check_if_pangram(sentence: String) -> bool {
    use std::collections::HashSet;
    let list: Vec<char> = sentence.chars().collect();
    let mut sett: HashSet<char, _> = HashSet::new();

    for c in list {
        sett.insert(c);
    }
    
    sett.len() == 26
}
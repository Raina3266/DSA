use std::collections::HashMap;

pub fn repeated_character(s: String) -> char {
    let mut map: HashMap<char, usize> = HashMap::new();
    let string_into_list: Vec<char> = s.chars().collect();

    for (index, c) in string_into_list.into_iter().enumerate(){
        if map.contains_key(&c) {
            return c;
        }
        map.insert(c, index);
    }

    unreachable!()
}

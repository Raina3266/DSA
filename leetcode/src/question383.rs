
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;
    let mut map_mag: HashMap<char, i32> = HashMap::new();
    let char_mag: Vec<char> = magazine.chars().collect();
    let char_ran: Vec<char> = ransom_note.chars().collect();
    for c in char_mag {
        *map_mag.entry(c).or_insert(0) += 1;
    }
    for r in char_ran {
        if map_mag.contains_key(&r) {
            let value = map_mag.get_mut(&r).unwrap();
            *value -= 1;
            if *value == -1 {
                return false;
            } 
        } else {
            return false;
        }
    }
    true
}
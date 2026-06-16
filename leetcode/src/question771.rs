use std::collections::HashMap;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewel_iter: Vec<char> = jewels.chars().collect();
        let stone_iter: Vec<char>  = stones.chars().collect();
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut count = 0;
    for s in stone_iter {
        *map.entry(s).or_insert(0) += 1;
    }
    for j in jewel_iter {
        if let Some(&stone_count) = map.get(&j) { count += &stone_count }
    }
    count as i32
}


use std::{cmp::min, collections::HashMap};

pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut ans = cards.len() as i32 + 1;
    for (index, num) in cards.iter().enumerate() {
        if map.contains_key(num){
            ans = min(ans, (index - map.get(num).unwrap() + 1) as i32);
        } 
        map.insert(*num, index);
    }
    
    if ans == (cards.len() + 1) as i32 {
        ans = -1;
    }
    ans
}
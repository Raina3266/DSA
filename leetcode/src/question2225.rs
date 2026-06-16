

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, HashSet};
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut winners: HashSet<i32> = HashSet::new();

    for q in &matches {
        winners.insert(q[0]);
        *map.entry(q[1]).or_insert(0) += 1
    }

    for p in &matches {
        winners.remove(&p[1]);
    } 

    let mut vec1: Vec<i32> = winners.into_iter().collect();
    vec1.sort();
    let mut vec2: Vec<i32> = map.iter().filter(|&(_, &value)| value == 1).map(|(key,_)| *key).collect();
    vec2.sort();
    vec![vec1, vec2]
}
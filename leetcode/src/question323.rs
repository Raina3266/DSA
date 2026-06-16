use std::collections::{HashMap, HashSet};

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for x in edges {
        map.entry(x[0]).and_modify(|v| v.push(x[1])).or_insert(vec![x[1]]);
        map.entry(x[1]).and_modify(|v| v.push(x[0])).or_insert(vec![x[0]]);
    }
    
    let mut vec = vec![0; n.try_into().unwrap()];
    let mut count = 0;
    for i in 0..vec.len() {
        if vec[i] == 0 {
            count += 1;
            let mut sett: HashSet<i32> = HashSet::new();
            let result = all_edges_from_one_node(&map, &mut sett, i.try_into().unwrap());
            for r in result {
                vec[r as usize] = 1;
            }
        }
    }
    count
}

fn all_edges_from_one_node(map: &HashMap<i32, Vec<i32>>, sett: &mut HashSet<i32>, node: i32) -> HashSet<i32> {
    sett.insert(node);
    if let Some(edges) = map.get(&node) {
        if edges.is_empty() {
            return sett.clone();
        }
        for e in edges {
            if !sett.contains(e) {
                all_edges_from_one_node(map, sett, *e);
            }
        }
        sett.clone()
    } else {
        sett.clone()
    }
}
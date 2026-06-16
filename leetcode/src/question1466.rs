use std::collections::{HashMap, HashSet};

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in &connections {
        map.entry(edge[0])
            .and_modify(|v| v.push(edge[1]))
            .or_insert(vec![edge[1]]);
        map.entry(edge[1])
            .and_modify(|v| v.push(edge[0]))
            .or_insert(vec![edge[0]]);
    }
    let mut sett: HashSet<(i32, i32)> = HashSet::new();
    let mut single_sett: HashSet<i32> = HashSet::new();
    let pairs = all_connections_away_zero(0, &map, &mut sett, &mut single_sett);
    println!("{pairs:?}");
    let mut count = 0;
    for x in &connections {
        if pairs.contains(&(x[0], x[1])) {
            count += 1
        }
    }
    count
}

fn all_connections_away_zero(
    head: i32,
    map: &HashMap<i32, Vec<i32>>,
    sett: &mut HashSet<(i32, i32)>,
    single_sett: &mut HashSet<i32>,
) -> HashSet<(i32, i32)> {
    single_sett.insert(head);
    if let Some(edges) = map.get(&head) {
        if edges.is_empty() {
            return sett.clone();
        }
        for &e in edges {
            if !single_sett.contains(&e) && sett.insert((head, e)) {
                all_connections_away_zero(e, map, sett, single_sett);
            }
        }
        sett.clone()
    } else {
        sett.clone()
    }
}

#[test]
fn feature() {
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    min_reorder(6, connections);
}

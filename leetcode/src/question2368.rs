use std::collections::{HashMap, HashSet};

pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in &edges {
        map.entry(edge[0])
            .and_modify(|v| v.push(edge[1]))
            .or_insert(vec![edge[1]]);
        map.entry(edge[1])
            .and_modify(|v| v.push(edge[0]))
            .or_insert(vec![edge[0]]);
    }

    let mut single_sett: HashSet<i32> = HashSet::from_iter(restricted);
    let mut result = 1;
    all_connections_away_zero(&mut result, 0, &map, &mut single_sett)
}

fn all_connections_away_zero(
    result: &mut i32,
    head: i32,
    map: &HashMap<i32, Vec<i32>>,
    single_sett: &mut HashSet<i32>,
) -> i32 {
    single_sett.insert(0);
    if let Some(edges) = map.get(&head) {
        for edge in edges {
            if !single_sett.contains(edge) {
                single_sett.insert(*edge);
                *result += 1;
                all_connections_away_zero(result, *edge, map, single_sett);
            }
        }
    }
    *result
}

#[test]
fn feature() {
    let vec = vec![
        vec![0, 1],
        vec![1, 2],
        vec![3, 1],
        vec![4, 0],
        vec![0, 5],
        vec![5, 6],
    ];
    assert_eq!(reachable_nodes(7, vec, vec![4, 5]), 4)
}

// fn all_connections_away_zero(
//     result: &mut HashSet<(i32, i32)>,
//     head: i32,
//     map: &HashMap<i32, Vec<i32>>,
//     single_sett: &mut HashSet<i32>,
// ) -> HashSet<(i32, i32)> {
//     single_sett.insert(head);
//     if let Some(edges) = map.get(&head) {
//         if edges.is_empty() {
//             return result.clone();
//         }
//         for &e in edges {
//             if !single_sett.contains(&e) {
//                 result.insert((head, e));
//                 all_connections_away_zero(result, e, map, single_sett);
//             }
//         }
//         result.clone()
//     } else {
//         result.clone()
//     }
// }

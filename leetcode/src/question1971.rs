use std::collections::{HashMap, HashSet};

struct Graph {
    nodes: HashSet<i32>,
    edges: HashMap<i32, HashSet<i32>>,
}

impl Graph {
    fn can_reach(&self, f: impl Fn() -> bool) {
        todo!()
    }
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for x in 0..edges.len() {
        map.entry(edges[x][0])
            .and_modify(|v| v.push(edges[x][1]))
            .or_insert(vec![edges[x][1]]);
        map.entry(edges[x][1])
            .and_modify(|v| v.push(edges[x][0]))
            .or_insert(vec![edges[x][0]]);
    }
    println!("{map:?}");
    if source == destination {
        return true;
    }
    let mut sett: HashSet<i32> = HashSet::new();
    check_pass(&map, source, destination, &mut sett)
}

fn check_pass(
    map: &HashMap<i32, Vec<i32>>,
    source: i32,
    destination: i32,
    sett: &mut HashSet<i32>,
) -> bool {
    sett.insert(source);
    if let Some(edges) = map.get(&source) {
        if edges.is_empty() {
            return sett.contains(&destination);
        }
        for v in edges {
            if !sett.contains(v) {
                check_pass(map, *v, destination, sett);
            }
        }
        sett.contains(&destination)
    } else {
        false
    }
}

#[test]
fn feature() {
    let edges = vec![
        vec![4, 3],
        vec![1, 4],
        vec![4, 8],
        vec![1, 7],
        vec![6, 4],
        vec![4, 2],
        vec![7, 4],
        vec![4, 0],
        vec![0, 9],
        vec![5, 4],
    ];
    assert_eq!(valid_path(10, edges, 5, 9), true);
}

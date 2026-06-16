use std::collections::HashMap;

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // keys are starting nodes, value is "nodes accessible from key"
    let mut edges: HashMap<usize, Vec<i32>> = HashMap::new();

    for (i, nums) in graph.into_iter().enumerate() {
        edges.insert(i, nums);
    }

    let edges_0 = edges.get(&0).unwrap();

    let mut results: Vec<Vec<i32>> = vec![];
    let ans = split(&edges, edges_0, &mut results);

    println!("{ans:?}");
    todo!()
}

fn split(map: &HashMap<usize, Vec<i32>>, edges: &[i32], results: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (first, second) = edges.split_first().unwrap();
    if second.is_empty() {
        return results.to_vec();
    } 
    let mut result: Vec<i32> = vec![];
    results.push(start_to_end(map, *first as usize, &mut result));
    let mut x = split(map, second, results);
    results.append(&mut x);
    results.to_vec()

}

fn start_to_end(
    map: &HashMap<usize, Vec<i32>>,
    start: usize,
    result: &mut Vec<i32>,
) -> Vec<i32> {
    result.push(start as i32);
    let edges = map.get(&start).unwrap();

    if edges.is_empty() {
        return result.clone();
    } else {
        for edge in edges {
            start_to_end(map, *edge as usize, result);
        }
    }
    result.clone()
}

#[test]
fn feature() {
    let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
    assert_eq!(all_paths_source_target(graph), vec![vec![]]);
}

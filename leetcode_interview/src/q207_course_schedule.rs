use std::collections::{HashMap, HashSet};

struct Requirement {
    course: i32,
    require: i32,
}

fn can_finish(prerequisites: Vec<Vec<i32>>) -> bool {
    let edges = make_edges(&prerequisites);
    for prerequisite in prerequisites {
        if depend_on(&edges, prerequisite[1], prerequisite[0]) {
            return false;
        }
    }
    true
}

// a depends on b intransitly
fn depend_on(edges: &HashMap<i32, HashSet<i32>>, a: i32, b: i32) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut to_visit = vec![a];
    while let Some(pop) = to_visit.pop() {
        if pop == b {
            return true;
        }
        if !visited.insert(pop) {
            continue;
        }
        if let Some(values) = edges.get(&pop) {
            for value in values {
                to_visit.push(*value);
            }
        }
    }
    false
}

fn make_edges(prerequisites: &Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut result: HashMap<i32, HashSet<i32>> = HashMap::new();
    for prerequisite in prerequisites {
        if let Some(values) = result.get_mut(&prerequisite[0]) {
            values.insert(prerequisite[1]);
        } else {
            result.insert(prerequisite[0], HashSet::from_iter([prerequisite[1]]));
        }
    }
    result
}

#[test]
fn feature() {
    let pre1 = vec![vec![0, 1], vec![1, 2], vec![2, 1]];
    // let pre2 = vec![vec![0, 1], vec![1, 2]];
    // let pre3 = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
    // let pre4 = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]];

    assert!(!can_finish(pre1));
    // assert!(can_finish(pre2));
    // assert!(can_finish(pre3));
    // assert!(!can_finish(pre4));
}

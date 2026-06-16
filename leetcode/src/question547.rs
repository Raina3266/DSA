use std::collections::{HashMap, HashSet};

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for x in 0..is_connected.len() {
        map.entry((x + 1) as i32).or_default();
        for y in 0..is_connected[x].len() {
            if is_connected[x][y] == 1 && x != y {
                map.entry((x + 1) as i32)
                    .and_modify(|v| v.push((y + 1) as i32));
            }
        }
    }
    let mut seen = vec![0; is_connected.len()];
    let mut count = 0;
    for i in 0..is_connected.len() {
        if seen[i] == 0 {
            count += 1;
            let mut sett: HashSet<i32> = HashSet::new();
            let result = has_seen((i + 1).try_into().unwrap(), &map, &mut sett);
            for s in result {
                seen[(s - 1) as usize] = 1
            }
        }
    }
    count
}

fn has_seen(
    current_point: i32,
    map: &HashMap<i32, Vec<i32>>,
    sett: &mut HashSet<i32>,
) -> HashSet<i32> {
    sett.insert(current_point);
    let edges = map.get(&(current_point as i32)).unwrap();
    if edges.is_empty() {
        return sett.clone();
    }
    for v in edges {
        if !sett.contains(v) {
            has_seen(*v, map, sett);
        }
    }
    sett.clone()
}

#[test]
fn feature() {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut sett: HashSet<i32> = HashSet::new();
    let current_point = 1;
    map.insert(3, vec![2, 4]);
    map.insert(1, vec![4]);
    map.insert(4, vec![1, 3]);
    map.insert(2, vec![3]);
    let result = has_seen(current_point, &map, &mut sett);
    let print: Vec<&i32> = result.iter().collect();
    println!("{sett:?}");
}

use std::collections::HashMap;

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    if nums.len() == 1 {
        return vec![nums];
    }
    let (head, tail) = nums.split_first().unwrap();
    for p in permute(tail.to_vec()) {
        let permutions = insert_head(*head, p);
        ans.extend(permutions);
    }
    ans
}

fn insert_head(head: i32, tail: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut tail = tail;
    for i in 0..tail.len() {
        tail.insert(i, head);
        ans.push(tail.clone());
        tail.remove(i);
    }
    tail.push(head);
    ans.push(tail.clone());
    ans
}

#[test]
fn feature() {
    let head = 1;
    let tail = vec![2, 3, 4];
    let result = vec![
        vec![1, 2, 3, 4],
        vec![2, 1, 3, 4],
        vec![2, 3, 1, 4],
        vec![2, 3, 4, 1],
    ];
    assert_eq!(insert_head(head, tail), result);
}

#[derive(PartialEq)]
enum Json {
    Null, // 1 byte
    Bool(bool), // 1 byte
    Number(i64),  // 8 bytes
    String(String), // length bytes
    Array(Vec<Json>),  // 24 bytes + children
    Map(HashMap<String, Json>)  // 48 bytes + total length of keys and values
}

fn total_memory_usage(json: &Json) -> usize {
    match json {
        Json::Null => return 1,
        Json::Bool(_) => return 1,
        Json::Number(_) => return 8,
        Json::String(x) => return x.len(),
        Json::Array(vec) => return memory_usage_for_array(vec),
        Json::Map(hash_map) => return memory_usage_for_hashmap(hash_map),
    }
    todo!()
}

fn memory_usage_for_array(list: &[Json]) -> usize {
    if list.is_empty() {
        return 24;
    } 
    let (head, tail) = list.split_first().unwrap();
    total_memory_usage(head) + memory_usage_for_array(tail)
}

fn memory_usage_for_hashmap(map: &HashMap<String, Json>) -> usize {
    let mut ans = 48;
    for (k, v) in map {
        ans += k.len() + total_memory_usage(v);
    }
    ans
}
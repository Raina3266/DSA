use std::collections::HashMap;

pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut map: HashMap<i32, usize> = HashMap::new();
    for v in nums {
        for i in v {
            *map.entry(i).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<i32> = map
        .iter()
        // Iterator<Item = (&i32, &usize)>
        .filter(|&(_, &value)| value == n)
        // Iterator<Item = (&i32, &usize)>
        .map(|(key, _)| *key) // fn(A) -> B  + Iterator<Item = A> = Iterator<Item = B>
        // Iterator<Item = i32>
        .collect();
    ans.sort();
    ans
}

// map.iter()  - Iterator<Item = &(i32, String)>
// map.into_iter()

use std::collections::HashMap;

pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    let x = map.iter().filter(|&(_, &value)| value == 1).map(|(key, _)| key).max().unwrap_or(&-1);
    *x
}

fn foo() {
    let nums = vec![1, 2, 3, 4, 5];

    // Iterator<Item = A>
    // .map(fn(A) -> B)
    // Iterator<Item = B>

    // Iterator<Item = A>
    // .max()
    // Option<A>

    // Iterator<Item = A>
    // .max_by_key(fn(A) -> B)
    // Option<A>

    // Iterator<Item = A>
    // .map(fn(A) -> B).max()
    // Option<B>

    let x = nums.iter().max_by_key(|&a| 10 - a).unwrap();
    println!("{x}");
}



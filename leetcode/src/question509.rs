// 0, 1, 1, 2, 3, 5, 8, 13, 21, ...

// to get the next number, you add the previous 2

// write a function ⁠ fn fib(n: i32) -> i32 ⁠ that calculates the nth fibonacci number:

//  - fib(0) = 0
//  - fib(1) = 1
//  - fib(2) = 1
//  - ...
//  - fib(8) = 21

pub fn fib(n: i32) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    fib_impl(n, map)
}

use std::collections::HashMap;
fn fib_impl(n: i32, mut map: HashMap<i32, i32>) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if let Some(value) = map.get(&n) {
                return *value;
            }
            let answer = fib_impl(n - 1, map.clone()) + fib_impl(n - 2, map.clone());
            map.insert(n, answer);
            answer
        }
    }
}

fn fib_fast(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

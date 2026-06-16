use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    let mut memory: HashMap<i32, i32> = HashMap::new();
    climb_stairs_recur(&mut memory, n)
}

fn climb_stairs_recur(memory: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if n == 1 {
        return 1;
    };
    if n == 2 {
        return 2;
    }
    if memory.contains_key(&n) {
        return *memory.get(&n).unwrap();
    } else {
        let ans = climb_stairs_recur(memory,n - 1) + climb_stairs_recur(memory,n - 2);
        memory.insert(n, ans);
        return ans
    }
}
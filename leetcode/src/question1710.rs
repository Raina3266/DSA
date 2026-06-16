use std::collections::HashMap;

pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    // (units, vec<nums of boxes>)
    let mut truck_size = truck_size;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for pair in box_types {
        map.entry(pair[1])
            .and_modify(|v| v.push(pair[0]))
            .or_insert(vec![pair[0]]);
    }
    let mut keys = vec![];
    for m in &map {
        keys.push(*m.0);
    }
    keys.sort_by(|a, b| b.cmp(a));
    let mut ans: Vec<(i32, i32)> = vec![];

    for i in keys {
        let value = map.get(&i).unwrap().clone();

        let sum: i32 = value.iter().sum();
        truck_size -= sum;

        if truck_size < 0 {
            truck_size += sum;
            ans.push((i, truck_size));

            return ans.iter().map(|(a, b)| a * b).sum::<i32>();
        } else {
            ans.push((i, sum));
        }
    }

    ans.iter().map(|(a, b)| a * b).sum::<i32>()
}
// 77 + 14
#[test]
fn feature() {
    let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
    let truck_size = 10;
    assert_eq!(maximum_units(box_types, truck_size), 91);
}

use std::collections::HashMap;


pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut map = HashMap::new();
    coin_change_2(&coins, amount,&mut map).unwrap_or(-1)
}

// [5,6,7] 100
fn coin_change_2(coins: &[i32], amount: i32, map: &mut HashMap<i32, Option<i32>>) -> Option<i32> {
    if map.contains_key(&amount) {
        return *map.get(&amount).unwrap();
    }
    if amount == 0 {
        return Some(0);
    } 
    let mut vec = vec![];
    for coin in coins {
        
        let previous = coin_change_2(coins, amount - coin, map);
       
        if let Some(i) = previous {
            vec.push(i);

        };
    }
    let result = vec.into_iter().min();
    map.insert(amount, result);
    result
}

#[test]
fn feature() {
    let coins = vec![186, 419, 83, 408];
    assert_eq!(coin_change(coins, 6249), 20);
}

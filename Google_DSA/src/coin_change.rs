use std::collections::HashMap;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    if amount < 0 {
        return -1;
    }
    let mut min_coins = i32::MAX;

    for coin in coins {
        let new_amount = amount - coin;
        coin_change(coins, new_amount);
    }
    if min_coins == i32::MAX { -1 } else { min_coins }
}

fn coin_change_dp(coins: Vec<i32>, amount: i32, cache: &mut HashMap<i32, i32>) -> i32 {}

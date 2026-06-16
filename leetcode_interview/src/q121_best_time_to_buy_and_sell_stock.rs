// finish with the best solution within 5 mins, no need to revisit

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lowest = prices[0];
    for price in prices {
        if price < lowest {
            lowest = price;
        }
        max_profit = max_profit.max(price - lowest);
    }
    max_profit
}

// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

pub fn max_profit(prices: &[i32]) -> i32 {
    let mut min = prices[0];
    let mut profit = 0;

    for &price in prices.iter().skip(1) {
        if price < min {
            min = price;
        } else {
            profit = std::cmp::max(profit, price - min);
        }
    }
    profit
}

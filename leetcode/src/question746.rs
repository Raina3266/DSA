pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    // nth_stair_cost[i] is the total cost to climb the first i stairs
    let mut nth_stair_cost = vec![0; cost.len() + 1];

    nth_stair_cost[0] = 0;
    nth_stair_cost[1] = 0;


    for i in 2..cost.len() + 1 {
        nth_stair_cost[i] = std::cmp::min(
            nth_stair_cost[i - 1] + cost[i - 1],
            nth_stair_cost[i - 2] + cost[i - 2],
        );
    }
    nth_stair_cost[cost.len()]
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = vec![0];
    for i in 1..temperatures.len() {
        // Empty means temperatures[i] is currently the warmest.
        if stack.is_empty() {
            stack.push(i);
        } 
        while !stack.is_empty() && temperatures[*stack.last().unwrap() as usize] < temperatures[i] {
            let u = stack.pop().unwrap();
            ans[u] = (i - u) as i32;     
        }
        stack.push(i);
    }
    ans
}
    

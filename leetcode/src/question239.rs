use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut queue: VecDeque<usize> = VecDeque::new();

    if k == nums.len() as i32 {
        let x = *nums.iter().max().unwrap();
        ans.push(x);
    } else {
        queue.push_front(0);
        for index in 1..nums.len() {
            if index < (k - 1) as usize {
                while !queue.is_empty() && nums[index] >= nums[*queue.back().unwrap()]  {
                    queue.pop_back();
                }
                queue.push_back(index)
            } else {
                while !queue.is_empty() && *queue.front().unwrap() < index + 1 - k as usize {
                    queue.pop_front();
                }
                while !queue.is_empty() && nums[index] >= nums[*queue.back().unwrap()] {
                    queue.pop_back();
                }
                queue.push_back(index);
                ans.push(nums[*queue.front().unwrap()]);
            }
        }
    }
    ans
}

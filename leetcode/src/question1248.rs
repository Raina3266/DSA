use std::collections::HashMap;

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
     let mut map:HashMap<i32, i32> = HashMap::with_capacity(2);
     let mut new_nums = vec![0; nums.len()];
     let mut count = 0;
     for n in 0..nums.len(){
          if nums[n] % 2 == 0 {
               new_nums[n] = 0;
          } else {
               new_nums[n] = 1;
          }
     }
     let mut prefix_sum = vec![0; new_nums.len()];
     prefix_sum[0] = new_nums[0];
     map.insert(prefix_sum[0], 1);
     if prefix_sum[0] == k {
          count = 1
     } 

     for i in 1..new_nums.len() {
          prefix_sum[i] = prefix_sum[i - 1] + new_nums[i];
          if prefix_sum[i] == k {
               count += 1
          }
          count += map.get(&(prefix_sum[i] - k)).unwrap_or(&0);
          *map.entry(prefix_sum[i]).or_insert(0) += 1
     }
     count
}
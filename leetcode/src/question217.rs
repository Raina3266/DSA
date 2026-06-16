// Given an integer array nums, return true if any value appears at least twice in the array, 
// and return false if every element is distinct.

use std::collections::HashSet;

pub fn contains_duplicate(nums: &[i32]) -> bool {
   let mut set = HashSet::with_capacity(nums.len());
   for n in nums {
        if set.contains(n) {
            return true;
        }
        set.insert(*n);
   }  
   false
}
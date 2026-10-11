//! 18. 4Sum — https://leetcode.com/problems/4sum/
//!
//! Given an array `nums` of `n` integers, return an array of all the unique
//! quadruplets `[nums[a], nums[b], nums[c], nums[d]]` such that:
//!   * 0 <= a, b, c, d < n
//!   * a, b, c, and d are distinct.
//!   * nums[a] + nums[b] + nums[c] + nums[d] == target
//!
//! You may return the answer in any order.
//!
//! Example 1:
//!   Input: nums = [1,0,-1,0,-2,2], target = 0
//!   Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//!
//! Example 2:
//!   Input: nums = [2,2,2,2,2], target = 8
//!   Output: [[2,2,2,2]]
//!
//! Constraints:
//!   * 1 <= nums.length <= 200
//!   * -10^9 <= nums[i] <= 10^9
//!   * -10^9 <= target <= 10^9

#![allow(dead_code, unused_variables)]

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sort each quadruplet and the outer list so results can be compared
    /// regardless of ordering.
    fn normalize(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for quad in result.iter_mut() {
            quad.sort_unstable();
        }
        result.sort_unstable();
        result
    }

    #[test]
    fn example_1() {
        assert_eq!(
            normalize(four_sum(vec![1, 0, -1, 0, -2, 2], 0)),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn example_2_all_same_value() {
        assert_eq!(
            normalize(four_sum(vec![2, 2, 2, 2, 2], 8)),
            vec![vec![2, 2, 2, 2]]
        );
    }

    #[test]
    fn exactly_four_elements_matching() {
        assert_eq!(
            normalize(four_sum(vec![1, 2, 3, 4], 10)),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn fewer_than_four_elements() {
        assert!(four_sum(vec![1, 2, 3], 6).is_empty());
    }

    #[test]
    fn negative_target() {
        assert_eq!(
            normalize(four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11)),
            vec![vec![-5, -4, -3, 1]]
        );
    }
}

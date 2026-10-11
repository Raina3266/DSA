//! 15. 3Sum — https://leetcode.com/problems/3sum/
//!
//! Given an integer array `nums`, return all the triplets `[nums[i], nums[j], nums[k]]`
//! such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
//!
//! Notice that the solution set must not contain duplicate triplets.
//!
//! Example 1:
//!   Input: nums = [-1,0,1,2,-1,-4]
//!   Output: [[-1,-1,2],[-1,0,1]]
//!   Explanation:
//!   nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//!   nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//!   nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//!   The distinct triplets are [-1,0,1] and [-1,-1,2].
//!   Notice that the order of the output and the order of the triplets does not matter.
//!
//! Example 2:
//!   Input: nums = [0,1,1]
//!   Output: []
//!   Explanation: The only possible triplet does not sum up to 0.
//!
//! Example 3:
//!   Input: nums = [0,0,0]
//!   Output: [[0,0,0]]
//!   Explanation: The only possible triplet sums up to 0.
//!
//! Constraints:
//!   * 3 <= nums.length <= 3000
//!   * -10^5 <= nums[i] <= 10^5

#![allow(dead_code, unused_variables)]

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sort each triplet and the outer list so results can be compared
    /// regardless of ordering.
    fn normalize(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in result.iter_mut() {
            triplet.sort_unstable();
        }
        result.sort_unstable();
        result
    }

    #[test]
    fn example_1() {
        assert_eq!(
            normalize(three_sum(vec![-1, 0, 1, 2, -1, -4])),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn example_2_no_triplet_sums_to_zero() {
        assert!(three_sum(vec![0, 1, 1]).is_empty());
    }

    #[test]
    fn example_3_all_zeros() {
        assert_eq!(normalize(three_sum(vec![0, 0, 0])), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn all_positive_no_solution() {
        assert!(three_sum(vec![1, 2, 3]).is_empty());
    }

    #[test]
    fn deduplicates_triplets() {
        assert_eq!(
            normalize(three_sum(vec![-2, 0, 1, 1, 2])),
            vec![vec![-2, 0, 2], vec![-2, 1, 1]]
        );
    }
}

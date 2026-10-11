//! 207. Course Schedule — https://leetcode.com/problems/course-schedule/
//!
//! There are a total of `numCourses` courses you have to take, labeled from `0`
//! to `numCourses - 1`. You are given an array `prerequisites` where
//! `prerequisites[i] = [ai, bi]` indicates that you must take course `bi` first
//! if you want to take course `ai`.
//!
//!   * For example, the pair [0, 1], indicates that to take course 0 you have to
//!     first take course 1.
//!
//! Return `true` if you can finish all courses. Otherwise, return `false`.
//!
//! Example 1:
//!   Input: numCourses = 2, prerequisites = [[1,0]]
//!   Output: true
//!   Explanation: There are a total of 2 courses to take.
//!   To take course 1 you should have finished course 0. So it is possible.
//!
//! Example 2:
//!   Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
//!   Output: false
//!   Explanation: There are a total of 2 courses to take.
//!   To take course 1 you should have finished course 0, and to take course 0 you
//!   should also have finished course 1. So it is impossible.
//!
//! Constraints:
//!   * 1 <= numCourses <= 2000
//!   * 0 <= prerequisites.length <= 5000
//!   * prerequisites[i].length == 2
//!   * 0 <= ai, bi < numCourses
//!   * All the pairs prerequisites[i] are unique.

#![allow(dead_code, unused_variables)]

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_possible() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn example_2_cycle_impossible() {
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn no_prerequisites() {
        assert!(can_finish(3, vec![]));
    }

    #[test]
    fn self_loop_is_impossible() {
        assert!(!can_finish(1, vec![vec![0, 0]]));
    }

    #[test]
    fn longer_chain_possible() {
        // 0 -> 1 -> 2 -> 3
        assert!(can_finish(4, vec![vec![1, 0], vec![2, 1], vec![3, 2]]));
    }
}

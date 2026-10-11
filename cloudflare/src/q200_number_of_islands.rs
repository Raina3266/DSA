//! 200. Number of Islands — https://leetcode.com/problems/number-of-islands/
//!
//! Given an `m x n` 2D binary grid `grid` which represents a map of `'1'`s (land)
//! and `'0'`s (water), return the number of islands.
//!
//! An island is surrounded by water and is formed by connecting adjacent lands
//! horizontally or vertically. You may assume all four edges of the grid are all
//! surrounded by water.
//!
//! Example 1:
//!   Input: grid = [
//!     ["1","1","1","1","0"],
//!     ["1","1","0","1","0"],
//!     ["1","1","0","0","0"],
//!     ["0","0","0","0","0"]
//!   ]
//!   Output: 1
//!
//! Example 2:
//!   Input: grid = [
//!     ["1","1","0","0","0"],
//!     ["1","1","0","0","0"],
//!     ["0","0","1","0","0"],
//!     ["0","0","0","1","1"]
//!   ]
//!   Output: 3
//!
//! Constraints:
//!   * m == grid.length
//!   * n == grid[i].length
//!   * 1 <= m, n <= 300
//!   * grid[i][j] is '0' or '1'.

#![allow(dead_code, unused_variables)]

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_one_island() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn example_2_three_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn all_water() {
        let grid = vec![vec!['0', '0'], vec!['0', '0']];
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn all_land_single_island() {
        let grid = vec![vec!['1', '1'], vec!['1', '1']];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn single_cell() {
        assert_eq!(num_islands(vec![vec!['1']]), 1);
    }
}

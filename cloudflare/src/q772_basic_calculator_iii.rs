//! 772. Basic Calculator III — https://leetcode.com/problems/basic-calculator-iii/
//! (premium; hardest extension: parentheses + all four operators)
//!
//! Implement a basic calculator to evaluate a simple expression string.
//!
//! The expression string contains only non-negative integers, `'+'`, `'-'`,
//! `'*'`, `'/'` operators, and open `'('` and closing parentheses `')'`. The
//! integer division should truncate toward zero.
//!
//! You may assume that the given expression is always valid. All intermediate
//! results will be in the range of `[-2^31, 2^31 - 1]`.
//!
//! Note: You are not allowed to use any built-in function which evaluates strings
//! as mathematical expressions, such as `eval()`.
//!
//! Example 1:
//!   Input: s = "1+1"
//!   Output: 2
//!
//! Example 2:
//!   Input: s = "6-4/2"
//!   Output: 4
//!
//! Example 3:
//!   Input: s = "2*(5+5*2)/3+(6/2+8)"
//!   Output: 21
//!
//! Constraints:
//!   * 1 <= s.length <= 10^4
//!   * s consists of digits, '+', '-', '*', '/', '(', and ')'.
//!   * s is a valid expression.

#![allow(dead_code, unused_variables)]

pub fn calculate(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(calculate("1+1".to_string()), 2);
    }

    #[test]
    fn example_2_division_before_subtraction() {
        assert_eq!(calculate("6-4/2".to_string()), 4);
    }

    #[test]
    fn example_3_parentheses_and_precedence() {
        assert_eq!(calculate("2*(5+5*2)/3+(6/2+8)".to_string()), 21);
    }

    #[test]
    fn negative_intermediate_result() {
        assert_eq!(calculate("(2+6*3+5-(3*14/7+2)*5)+3".to_string()), -12);
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(calculate("7/2".to_string()), 3);
    }
}

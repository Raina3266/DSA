//! 227. Basic Calculator II — https://leetcode.com/problems/basic-calculator-ii/
//! (★ confirmed phone screen)
//!
//! Given a string `s` which represents an expression, evaluate this expression
//! and return its value.
//!
//! The integer division should truncate toward zero.
//!
//! You may assume that the given expression is always valid. All intermediate
//! results will be in the range of `[-2^31, 2^31 - 1]`.
//!
//! Note: You are not allowed to use any built-in function which evaluates strings
//! as mathematical expressions, such as `eval()`.
//!
//! Example 1:
//!   Input: s = "3+2*2"
//!   Output: 7
//!
//! Example 2:
//!   Input: s = " 3/2 "
//!   Output: 1
//!
//! Example 3:
//!   Input: s = " 3+5 / 2 "
//!   Output: 5
//!
//! Constraints:
//!   * 1 <= s.length <= 3 * 10^5
//!   * s consists of integers and operators ('+', '-', '*', '/') separated by some
//!     number of spaces.
//!   * s represents a valid expression.
//!   * All the integers in the expression are non-negative integers in the range
//!     [0, 2^31 - 1].
//!   * The answer is guaranteed to fit in a 32-bit integer.

enum Operant {
    Add,
    Min,
    Mul,
    Div,
}

impl Operant {
    fn parse(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Min,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => unreachable!(),
        }
    }
}

pub fn calculate(s: String) -> i32 {
    let expr: Vec<char> = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    calculate_expr(&expr)
}

fn calculate_expr(s: &[char]) -> i32 {
    let (mut num, mut operant) = get_number(s, 0);
    while operant != s.len() {
        match Operant::parse(s[operant]) {
            Operant::Add => return num + calculate_expr(&s[operant + 1..]),
            Operant::Min => return num - calculate_expr(&s[operant + 1..]),
            Operant::Mul => {
                let (n, next) = get_number(s, operant + 1);
                num *= n;
                operant = next;
            }
            Operant::Div => {
                let (n, next) = get_number(s, operant + 1);
                num /= n;
                operant = next;
            }
        }
    }
    num
}

fn get_number(s: &[char], start: usize) -> (i32, usize) {
    let mut curr = start;
    while curr < s.len() - 1 && s[curr + 1].is_ascii_digit() {
        curr += 1;
    }

    (
        s[start..=curr]
            .iter()
            .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap()) as i32,
        curr + 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(calculate("3+2*2".to_string()), 7);
    }

    #[test]
    fn example_2_division_truncates_toward_zero() {
        assert_eq!(calculate(" 3/2 ".to_string()), 1);
    }

    #[test]
    fn example_3_with_spaces() {
        assert_eq!(calculate(" 3+5 / 2 ".to_string()), 5);
    }

    #[test]
    fn subtraction_and_division() {
        assert_eq!(calculate("14-3/2".to_string()), 13);
    }

    #[test]
    fn single_number() {
        assert_eq!(calculate("42".to_string()), 42);
    }
}

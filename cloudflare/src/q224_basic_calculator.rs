//! 224. Basic Calculator — https://leetcode.com/problems/basic-calculator/
//! (extension drill: adds parentheses to Basic Calculator II)
//!
//! Given a string `s` representing a valid expression, implement a basic
//! calculator to evaluate it, and return the result of the evaluation.
//!
//! Note: You are not allowed to use any built-in function which evaluates strings
//! as mathematical expressions, such as `eval()`.
//!
//! Example 1:
//!   Input: s = "1 + 1"
//!   Output: 2
//!
//! Example 2:
//!   Input: s = " 2-1 + 2 "
//!   Output: 3
//!
//! Example 3:
//!   Input: s = "(1+(4+5+2)-3)+(6+8)"
//!   Output: 23
//!
//! Constraints:
//!   * 1 <= s.length <= 3 * 10^5
//!   * s consists of digits, '+', '-', '(', ')', and ' '.
//!   * s represents a valid expression.
//!   * '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
//!   * '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
//!   * There will be no two consecutive operators in the input.
//!   * Every number and running calculation will fit in a signed 32-bit integer.

#![allow(dead_code, unused_variables)]

struct Calculator {
    token: Vec<char>,
    start: usize,
    current: usize,
}

impl Calculator {
    fn new(s: String) -> Self {
        Calculator {
            token: s.chars().filter(|c| !c.is_ascii_whitespace()).collect(),
            start: 0,
            current: 0,
        }
    }
}

pub fn calculate(s: String) -> i32 {
    let mut c = Calculator::new(s);
    cal(&mut c)
}

fn cal(s: &mut Calculator) -> i32 {
    let mut num = 0;
    while s.current < s.token.len() {
        match s.token[s.current] {
            '0'..='9' => {
                num = get_number(s);
                s.current += 1;
            }
            '+' => {
                s.current += 1;
                s.start = s.current;
                return num + cal(s);
            }
            '-' => {
                s.current += 1;
                s.start = s.current;
                return num - cal(s);
            }
            '(' => {
                s.current += 1;
                s.start = s.current;
                return cal(s);
            }
            ')' => {
                s.current += 1;
                s.start = s.current;
                return num;
            }
            _ => unreachable!(),
        }
    }
    num
}

fn get_number(s: &mut Calculator) -> i32 {
    while !is_at_end(s) && s.token[s.current + 1].is_ascii_digit() {
        s.current += 1;
    }
    
    s.token[s.start..=s.current]
        .iter()
        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap()) as i32
}

fn is_at_end(s: &Calculator) -> bool {
    s.token.len() == s.current + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn example_2_with_spaces() {
        assert_eq!(calculate(" 2-1 + 2 ".to_string()), 3);
    }

    #[test]
    fn example_3_nested_parentheses() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }

    #[test]
    fn unary_minus_before_parentheses() {
        assert_eq!(calculate("-(2 + 3)".to_string()), -5);
    }

    #[test]
    fn unary_minus_inside_parentheses() {
        assert_eq!(calculate("1-(-2)".to_string()), 3);
    }
}

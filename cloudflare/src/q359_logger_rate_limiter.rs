//! 359. Logger Rate Limiter (finish)
//! https://leetcode.com/problems/logger-rate-limiter/
//!
//! Design a logger system that receives a stream of messages along with their
//! timestamps. Each unique message should only be printed at most every 10
//! seconds (i.e. a message printed at timestamp `t` will prevent other identical
//! messages from being printed until timestamp `t + 10`).
//!
//! All messages will come in chronological order. Several messages may arrive at
//! the same timestamp.
//!
//! Implement the `Logger` class:
//!   * `Logger()` — Initializes the `Logger` object.
//!   * `shouldPrintMessage(timestamp, message) -> bool` — Returns `true` if the
//!     `message` should be printed in the given `timestamp`, otherwise returns
//!     `false`.
//!
//! Example 1:
//!   Input:
//!   ["Logger", "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage",
//!    "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage"]
//!   [[], [1, "foo"], [2, "bar"], [3, "foo"], [8, "bar"], [10, "foo"], [11, "foo"]]
//!   Output: [null, true, true, false, false, false, true]
//!   Explanation:
//!   Logger logger = new Logger();
//!   logger.shouldPrintMessage(1, "foo");  // return true,
//!                                         // next allowed timestamp for "foo" is 1 + 10 = 11
//!   logger.shouldPrintMessage(2, "bar");  // return true,
//!                                         // next allowed timestamp for "bar" is 2 + 10 = 12
//!   logger.shouldPrintMessage(3, "foo");  // 3 < 11, return false
//!   logger.shouldPrintMessage(8, "bar");  // 8 < 12, return false
//!   logger.shouldPrintMessage(10, "foo"); // 10 < 11, return false
//!   logger.shouldPrintMessage(11, "foo"); // 11 >= 11, return true,
//!                                         // next allowed timestamp for "foo" is 11 + 10 = 21
//!
//! Constraints:
//!   * 0 <= timestamp <= 10^9
//!   * Every timestamp will be passed in non-decreasing order (chronological order).
//!   * 1 <= message.length <= 30
//!   * At most 10^4 calls will be made to shouldPrintMessage.

#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, hash_map::Entry};

pub struct Logger {
    logs: HashMap<String, i32>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: HashMap::new(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        match self.logs.entry(message) {
            Entry::Occupied(mut e) if timestamp - *e.get() >= 10 => {
                e.insert(timestamp);
                true
            }
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(timestamp);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(1, "foo".to_string()));
        assert!(logger.should_print_message(2, "bar".to_string()));
        assert!(!logger.should_print_message(3, "foo".to_string()));
        assert!(!logger.should_print_message(8, "bar".to_string()));
        assert!(!logger.should_print_message(10, "foo".to_string()));
        assert!(logger.should_print_message(11, "foo".to_string()));
    }

    #[test]
    fn exactly_ten_seconds_later_is_allowed() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(0, "a".to_string()));
        assert!(logger.should_print_message(10, "a".to_string()));
    }

    #[test]
    fn different_messages_are_independent() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(1, "foo".to_string()));
        assert!(logger.should_print_message(1, "bar".to_string()));
    }

    #[test]
    fn same_message_same_timestamp_is_blocked() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(5, "x".to_string()));
        assert!(!logger.should_print_message(5, "x".to_string()));
    }

    #[test]
    fn expiry_is_tracked_per_message() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(1, "foo".to_string())); // expires at 11
        assert!(logger.should_print_message(2, "bar".to_string())); // expires at 12
        assert!(logger.should_print_message(11, "foo".to_string()));
        assert!(!logger.should_print_message(11, "bar".to_string()));
    }
}

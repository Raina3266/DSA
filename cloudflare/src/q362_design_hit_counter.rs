//! 362. Design Hit Counter — https://leetcode.com/problems/design-hit-counter/
//! (★ confirmed asked onsite)
//!
//! Design a hit counter which counts the number of hits received in the past 5
//! minutes (i.e., the past 300 seconds).
//!
//! Your system should accept a `timestamp` parameter (in seconds granularity),
//! and you may assume that calls are being made to the system in chronological
//! order (i.e., `timestamp` is monotonically increasing). Several hits may arrive
//! roughly at the same time.
//!
//! Implement the `HitCounter` class:
//!   * `HitCounter()` — Initializes the object of the hit counter system.
//!   * `hit(timestamp)` — Records a hit that happened at `timestamp` (in seconds).
//!     Several hits may happen at the same `timestamp`.
//!   * `getHits(timestamp) -> i32` — Returns the number of hits in the past 5
//!     minutes from `timestamp` (i.e., the past 300 seconds).
//!
//! Example 1:
//!   Input:
//!   ["HitCounter", "hit", "hit", "hit", "getHits", "hit", "getHits", "getHits"]
//!   [[], [1], [2], [3], [4], [300], [300], [301]]
//!   Output: [null, null, null, null, 3, null, 4, 3]
//!   Explanation:
//!   HitCounter hitCounter = new HitCounter();
//!   hitCounter.hit(1);       // hit at timestamp 1.
//!   hitCounter.hit(2);       // hit at timestamp 2.
//!   hitCounter.hit(3);       // hit at timestamp 3.
//!   hitCounter.getHits(4);   // get hits at timestamp 4, return 3.
//!   hitCounter.hit(300);     // hit at timestamp 300.
//!   hitCounter.getHits(300); // get hits at timestamp 300, return 4.
//!   hitCounter.getHits(301); // get hits at timestamp 301, return 3.
//!
//! Constraints:
//!   * 1 <= timestamp <= 2 * 10^9
//!   * All the calls are being made to the system in chronological order
//!     (i.e., timestamp is monotonically increasing).
//!   * Several hits may arrive roughly at the same time.
//!   * At most 300 calls will be made to hit and getHits.
//!
//! Follow up: What if the number of hits per second could be huge? Does your
//! design scale?

#![allow(dead_code, unused_variables)]

pub struct HitCounter {}

impl HitCounter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn hit(&mut self, timestamp: i32) {
        todo!()
    }

    pub fn get_hits(&self, timestamp: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(2);
        counter.hit(3);
        assert_eq!(counter.get_hits(4), 3);
        counter.hit(300);
        assert_eq!(counter.get_hits(300), 4);
        assert_eq!(counter.get_hits(301), 3);
    }

    #[test]
    fn multiple_hits_at_same_timestamp() {
        let mut counter = HitCounter::new();
        counter.hit(5);
        counter.hit(5);
        counter.hit(5);
        assert_eq!(counter.get_hits(5), 3);
    }

    #[test]
    fn no_hits_returns_zero() {
        let counter = HitCounter::new();
        assert_eq!(counter.get_hits(10), 0);
    }

    #[test]
    fn hit_exactly_300_seconds_ago_is_excluded() {
        let mut counter = HitCounter::new();
        counter.hit(2);
        assert_eq!(counter.get_hits(301), 1); // 301 - 300 = 1 < 2, still counted
        assert_eq!(counter.get_hits(302), 0); // 302 - 300 = 2, expired
    }

    #[test]
    fn many_hits_within_window() {
        let mut counter = HitCounter::new();
        for t in 1..=10 {
            counter.hit(t);
        }
        assert_eq!(counter.get_hits(300), 10);
    }
}

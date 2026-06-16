use core::f64;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]

struct F64Ord(f64);

impl PartialEq for F64Ord {
    fn eq(&self, other: &F64Ord) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

// Eq is a marker trait, there are no functions inside it (or types, or consts)
// 3 rules:
//  - symmetry - a=b -> b=a
//  - transitivity - a=b, b=c -> a=c
//  - reflexivity - a=a  [Eq only]
impl Eq for F64Ord {}

impl PartialOrd for F64Ord {
    fn partial_cmp(&self, other: &F64Ord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F64Ord {
    fn cmp(&self, other: &F64Ord) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub fn halve_array(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let mut sum = 0.0;
    let mut nums_f64 = vec![];
    for i in 0..nums.len() {
        sum += nums[i] as f64;
        nums_f64.push(F64Ord(nums[i] as f64));
    }
    let mut heap: BinaryHeap<F64Ord> = BinaryHeap::from(nums_f64);
    let half = sum / 2.0;
    let mut ans = 0;

    while sum > half {
        let max = heap.pop().unwrap().0;
        heap.push(F64Ord(max / 2.0));
        ans += 1;
        sum -= max / 2.0;
    }
    ans
}

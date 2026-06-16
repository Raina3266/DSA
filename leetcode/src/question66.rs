struct Solution;

impl Solution {
    // [1,2,9,9]
    // [9]

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let range = 0..=digits.len() - 1;

        *digits.last_mut().unwrap() += 1;

        for x in range.rev() {
            if digits[x] == 10 {
                digits[x] = 0;
                if x == 0 {
                    digits.insert(0, 1);
                } else {
                    digits[x - 1] += 1;
                }
            }
        }
        digits
    }
}

#[test]
fn stupid_me() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3, 4]), [1, 2, 3, 5]);
    assert_eq!(Solution::plus_one(vec![1, 8, 9, 9]), [1, 9, 0, 0]);
    assert_eq!(Solution::plus_one(vec![9]), [1, 0])
}

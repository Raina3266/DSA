pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut piles = nums;
    let h_usize: usize = threshold.try_into().unwrap();
    piles.sort();
    let length = piles.len();
    if length == h_usize {
        return piles[length - 1];
    }
    if piles.len() == 1 {
        if piles[0] % threshold == 0 {
            return piles[0] / threshold;
        } else {
            return piles[0] / threshold + 1;
        }
    }

    let mut left = 1;
    let mut right = piles[length - 1];
    loop {
        if right - left <= 1 {
            if calculate_sum_for_one_divisor(&piles, left) <= threshold {
                return left;
            } else {
                return right;
            }
        }
        let midpoint = (right + left) / 2;

        if calculate_sum_for_one_divisor(&piles, midpoint) <= threshold {
            right = midpoint;
        } else {
            left = midpoint;
        }
    }
}

fn calculate_sum_for_one_divisor(nums: &[i32], divisor: i32) -> i32 {
    let mut sum = 0;
    for p in nums {
        if p % divisor == 0 {
            sum += p / divisor;
        } else {
            sum += p / divisor + 1;
        }
    }
    sum
}

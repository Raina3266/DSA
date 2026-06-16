pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    loop {
        let mid = (left + right) / 2;
        if right - left <= 1   {
            if nums[left] == target {
                return left as i32;
            } else if nums[right] == target {
                return right as i32;
            } else {
                return -1
            }
        }
    
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
}   

#[test]
fn feature() {
    assert_eq!(search(vec![-1,0,5], 5), 2)
}


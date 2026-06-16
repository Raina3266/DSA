// do it again

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; n];
    let mut product = 1;
    let mut zero_count = 0;

    for num in &nums {
        if *num == 0 {
            zero_count += 1;
            continue;
        }
        product *= num;
    }

    if zero_count == 1 {
        for i in 0..n {
            ans[i] = if nums[i] == 0 { product } else { 0 };
        }
    } else if zero_count == 0 {
        for i in 0..n {
            ans[i] = product / nums[i];
        }
    }
    ans
}

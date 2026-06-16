pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let mut number_of_subarray = 0;
    let mut left = 0;
    let mut product = 1;

    for right in 0..nums.len() {
        product *= nums[right];

        if product >= k {
            product /= nums[left];
            left += 1;
        }
        number_of_subarray += right - left + 1;
    }
    number_of_subarray as i32
}


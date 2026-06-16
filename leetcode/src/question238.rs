// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// You must write an algorithm that runs in O(n) time and without using the division operation.

// Copy from other

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let size = nums.len();
    let mut products = vec![0; size];
    products[0] = 1;
    let mut prefix = 1;
    for idx in 0..size - 1 {
        prefix *= nums[idx];
        products[idx + 1] = prefix;
    }
    let mut suffix = 1;
    for idx in (0..size).rev() {
        products[idx] *= suffix;
        suffix *= nums[idx];
    }
    products
}
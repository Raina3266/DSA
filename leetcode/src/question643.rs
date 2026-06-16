
pub fn biggest_average(nums: Vec<i32>, k: i32) -> f64 {

    let len = nums.len();
    let new_k: usize = k.try_into().unwrap();
    let f_k: f64 = k.into();
    let init_sum: i32 = nums[0..=new_k-1].iter().sum();
    let init_f64_sum: f64 = init_sum.into();
    let mut result = ((init_f64_sum / f_k) * 100000.0).round() / 100000.0;

    for index in 0..=len-new_k {
        let subarray = &nums[index..index + new_k];

        let sum = subarray.iter().sum::<i32>();
        let f64_sum = sum as f64;
        let average: f64 = f64_sum / f_k;
        let rounded_averge = (average * 100000.0).round() / 100000.0;
        if rounded_averge > result {
            result = rounded_averge
        }
    }
    result
} 

pub fn biggest_average_better(nums: &[i32], k: usize) -> f64 {
    let biggest_sum = nums.windows(k).map(|subslice| subslice.into_iter().sum::<i32>()).max().unwrap();
    return biggest_sum as f64 / k as f64
    } 

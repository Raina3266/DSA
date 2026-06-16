pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let empty = vec![];

    if nums.len() == 1 {
        ans.push(empty);

        return ans;
    }
    ans.push(nums.clone());

    for i in 0..ans.len() {
        split(nums.clone(), i);
        ans.push(vec![nums[i]]);

        ans.extend(subsets(nums.to_vec()));
    }

    ans
}

fn split(nums: Vec<i32>, i: usize) -> (i32, Vec<i32>) {
    let mut nums = nums;
    let num = nums[i];
    nums.remove(i);
    (num, nums)
}

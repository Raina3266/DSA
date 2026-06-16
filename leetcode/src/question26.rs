
//nums = [0,0,1,1,2,2,3,3,4] 9
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut number = nums[0];
    let mut index = 0;

    while index < nums.len() - 1 {
        if nums[index + 1] != number {
            index += 1;
            number = nums[index];
        } else {
            nums.remove(index);
        }
    }
    nums.len() as i32
}

#[test]
fn my_test(){ 
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let k = remove_duplicates(&mut nums);
    assert_eq!(k, 5);
    assert_eq!(nums, [0, 1, 2, 3, 4])
}
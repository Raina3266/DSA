

// nums: [0,1,2,2,3,0,4,2]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    while index < nums.len() {
        if nums[index] != val {
            index += 1;
        } else {
            nums.remove(index);
        }
    }
    nums.len() as i32
}

#[test]
fn my_test(){ 
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let k = remove_element(&mut nums, val);
    assert_eq!(k, 5);
    assert_eq!(nums, [0,1,3,0,4])
}
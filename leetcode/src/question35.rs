// x = null | bool | string | number | list<x> | map<string, x>

// pub fn search_insert(nums: &[i32], target: i32) -> i32 {
//     if nums.len() == 1 {
//        if target <= nums[0] {
//         return 0;
//        }
//         return 1;
//     }
    
//     let midpoint = nums.len() / 2;
//     let (start, end) = nums.split_at(midpoint);
    

//     if target <= start[midpoint - 1] {
//         search_insert(start, target)
//     } else {
//         search_insert(end, target) + midpoint
//     };

// }


// #[test]
// fn sfind_biggest_test() {
//     let nums = [10, 50, 80, 22, 4];
//     assert_eq!(find_biggest(&nums), 80);
    
// }

// #[test]
// fn check_split_at_behaviour() {
//    let nums = vec![1, 3, 5, 6, 9];
//    let result = search_insert(&nums, 9);
//    assert_eq!(result, 4);
// }
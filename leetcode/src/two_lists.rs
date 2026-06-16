

pub fn sort_two_list(nums1: &[i32], nums2: &[i32]) -> Vec<i32>{

    if nums1.is_empty() {
        return nums2.to_vec();
    }

    if nums2.is_empty() {
        return nums1.to_vec();
    }

    let mut one = 0;
    let mut two = 0;
    let mut ans = Vec::new();

    while one < nums1.len() && two < nums2.len() {
        if nums1[one] <= nums2[two] {
            ans.push(nums1[one]);
            one += 1;
        } else {
            ans.push(nums2[two]);
            two += 1;
        }
    }

    while one < nums1.len() {
        ans.push(nums1[one]);
        one += 1;
    } 

    while two < nums2.len() {
        ans.push(nums2[two]);
        two += 1;
    } 
    ans
}

// O(n * log(n)) where n is the combined size of the lists
fn sort_two_list_bad(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut v: Vec<_> = nums1.iter().chain(nums2).copied().collect();
    v.sort();
    v
}

#[test]
fn test() {
    let nums1 = &[1, 3, 5, 7, 9];
    let nums2 = &[6, 8, 9, 10, 13, 17, 22, 25, 26];
    assert_eq!(sort_two_list(nums1, nums2), [1, 3, 5, 6, 7, 8, 9, 9, 10, 13, 17, 22, 25, 26]);
}



#[test_strategy::proptest]
fn two_algorithms_should_give_same_answer(nums1: Vec<i32>, nums2: Vec<i32>) {
    let good_answer = sort_two_list(&nums1, &nums2);
    let bad_answer = sort_two_list_bad(&nums1, &nums2);

    //          [0, -1]       [-1, 0]
    assert_eq!(good_answer, bad_answer);
}

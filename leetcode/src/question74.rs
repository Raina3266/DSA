pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut left = 0;
    let mut right = matrix.len() - 1;
    loop {
        // 10,20,30
        let mid = (left + right) / 2;
        if left == right {
            return search_insert(&matrix[left], target);
        }
        if right - left == 1 {
            if matrix[right][0] <= target {
                return search_insert(&matrix[right], target);
            } else {
                return search_insert(&matrix[left], target);
            }
        }

        if matrix[mid][0] == target {
            return true;
        } else if matrix[mid][0] < target {
            left = mid;
        } else if matrix[mid][0] > target {
            right = mid - 1;
        }
    }
}

pub fn search_insert(nums: &[i32], target: i32) -> bool {
    if nums.len() == 1 {
        if target == nums[0] {
            return true;
        }
        return false;
    }
    let midpoint = nums.len() / 2;
    let (start, end) = nums.split_at(midpoint);
    if target <= start[midpoint - 1] {
        search_insert(start, target)
    } else {
        search_insert(end, target)
    }
}

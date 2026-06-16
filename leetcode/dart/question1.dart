void main() {
  final nums = [1, 2, 3, 4];
  final target = 7;
  final answer = twoSum(nums, target);
  print(answer);
}

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
List<int> twoSum(List<int> nums, int target) {
  for (var a = 0; a < nums.length; a++) {
    for (var b = a + 1; b < nums.length; b++) {
      if (nums[a] + nums[b] == target) {
        return [a, b];
      }
    }
  }
  throw Exception();
}

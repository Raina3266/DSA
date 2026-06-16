int removeElement(List<int> nums, int val) {
  var index = 0;
  while (index < nums.length) {
    if (nums[index] == val) {
      nums.removeAt(index);
    } else {
      index += 1;
    }
  }
  return nums.length;
}

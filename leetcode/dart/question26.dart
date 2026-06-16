void main() {
  final list = [1, 1, 2];
  removeDuplicates(list);
  print(list);
}

//nums = [0,0,1,1,2,2,3,3,4] 9
int removeDuplicates(List<int> nums) {
  var number = nums[0];
  var index = 0;

  while (index < nums.length - 1) {
    if (nums[index + 1] != number) {
      index += 1;
      number = nums[index];
    } else {
      nums.removeAt(index);
    }
  }
  return nums.length;
}

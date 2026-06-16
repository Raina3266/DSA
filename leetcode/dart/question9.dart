void main() {
  String reverse(String s) => s.split("").reversed.join();

  print(reverse("巭閄槑爨齉"));

  print(palindrome(12341254));
}

bool palindrome(int x) {
  String y = x.toString();
  String z = y.split("").reversed.join();
  if (y == z) {
    return true;
  }
  return false;
}





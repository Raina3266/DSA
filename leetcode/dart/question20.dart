void main() {}

bool isValid(String s) {
  final list = <String>[];
  for (final x in s.split("")) {
    if (x == '(') {
      list.add('(');
    } else if (x == '[') {
      list.add('[');
    } else if (x == '{') {
      list.add('{');
    } else if (x == '}') {
      if (list.isEmpty || list.removeLast() != '{') return false;
    } else if (x == ']') {
      if (list.isEmpty || list.removeLast() != '[') return false;
    } else if (x == ')') {
      if (list.isEmpty || list.removeLast() != '(') return false;
    }
  }
  
  return list.isEmpty;
}

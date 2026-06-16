void main() {}

String longestCommonPrefix(List<String> strs) {
  var prefix = strs[0];

  while (prefix.length != 0) {
    for (final singleString in strs) {
      if (startWithPrefix(strs, prefix)) {
        return prefix;
      } else {
        prefix = prefix.substring(0, prefix.length - 1);
      }
    }
  }
  return prefix;
}

bool startWithPrefix(List<String> strs, String prefix) {
  for (final everyString in strs) {
    final boo = everyString.startsWith(prefix);
    if (!boo) {
      return false;
    }
  }
  return true;
}

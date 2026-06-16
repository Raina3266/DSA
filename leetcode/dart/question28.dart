int strStr(String haystack, String needle) {
  for (var i = 0; i <= haystack.length; i++) {
    if (haystack.startsWith(needle, i)) {
      return i;
    }
  }
  return -1;
}

class Node<T> {
  final T data;
  final Node<T>? next;
  Node<T>? prev;
  Node(this.data, this.next) {
    next?.prev = this;
  }

  int get length => 1 + (next?.length ?? 0);
}

final node = Node(
    1,
    Node(
      2,
      null,
    ));

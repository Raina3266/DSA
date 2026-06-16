enum LinkedList<T> {
    Empty,
    Cons { head: T, tail: Box<LinkedList<T>> },
}

enum DoublyLinkedList<T> {
    Empty,
    Cons {
        head: T,
        prev: Box<DoublyLinkedList<T>>,
        next: Box<DoublyLinkedList<T>>,
    },
}

impl<T> LinkedList<T> {
    fn length(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Cons { tail, .. } => tail.length() + 1,
        }
    }
}

enum Tree<T> {
    Leaf(T),
    Parents {
        left: Option<Box<Tree<T>>>,
        right: Option<Box<Tree<T>>>,
    },
}

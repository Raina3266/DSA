use std::collections::HashMap;

struct TrieNode { 
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, word: String) {
        todo!()
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}
use std::collections::HashMap;


struct Trie {
    words: Vec<String>,
}

impl Trie {
    fn new() -> Self {
        Trie { words: Vec::new() }
    }

    fn insert(&mut self, word: String) {
        self.words.push(word);
    }

    fn search(&self, word: String) -> bool {
        self.words.iter().any(|w| w == &word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.words.iter().any(|word| word.starts_with(&prefix))
    }
}

// faster solution

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_leaf: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_leaf: false,
        }
    }
    fn insert(&mut self, word: String) {
        let chars = word.chars().collect::<Vec<char>>();
        let mut node = self;
        for char in chars {
            node = node.children.entry(char).or_insert(TrieNode::new());
        }
        node.is_leaf = true;
    }
    
    fn search(&self, word: String) -> bool {
        let chars = word.chars().collect::<Vec<char>>();
        let mut node = self;
        for char in chars {
            if let Some(n) = node.children.get(&char) {
                node = n;
            } else {
                return false;
            }
        }
        true
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let chars = prefix.chars().collect::<Vec<char>>();
        let mut node = self;
        for char in chars {
            if let Some(n) = node.children.get(&char) {
                node = n;
            } else {
                return false;
            }
        }
        true
    }
}

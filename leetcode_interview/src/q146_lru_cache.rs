use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: i32,
    pairs: HashMap<i32, i32>,
    frequency: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            pairs: HashMap::new(),
            frequency: VecDeque::new(),
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        match self.pairs.get(&key) {
            Some(value) => {
                self.frequency.retain(|x| x != &key);
                self.frequency.push_back(key);
                *value
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.pairs.insert(key, value).is_some() {
            self.frequency.retain(|x| x != &key);
            self.frequency.push_back(key);
        } else {
            if self.frequency.len() == self.capacity as usize {
                let key = self.frequency.pop_front().unwrap();
                self.pairs.remove(&key);
                self.frequency.push_back(key);
            } else {
                self.frequency.push_back(key);
            }
        }
    }
}

struct Cache {
    capacity: i32,
    pairs: HashMap<i32, i32>,
    frequency: Frequency,
}

enum Frequency {
    Empty,
    Cons {
        head: i32,
        prev: Box<Frequency>,
        next: Box<Frequency>,
    },
}

impl Frequency {
    fn length(&self) -> usize {
        match self {
            Frequency::Empty => todo!(),
            Frequency::Cons { next, .. } => 1 + next.length(),
        }
    }
    fn remove(&mut self, target: i32) {}

    fn add_to_back(&mut self, add: i32) {}
}

impl Cache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            pairs: HashMap::new(),
            frequency: Frequency::Empty,
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        match self.pairs.get(&key) {
            Some(value) => *value,
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.pairs.insert(key, value).is_some() {
            // self.frequency.retain(|x| x != &key);
            // self.frequency.push_back(key);
        } else {
            // if self.frequency.len() == self.capacity as usize {
            //     let key = self.frequency.pop_front().unwrap();
            //     self.pairs.remove(&key);
            //     self.frequency.push_back(key);
            // } else {
            //     self.frequency.push_back(key);
            // }
        }
    }
}

use std::collections::VecDeque;

struct RecentCounter {
    time: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            time: VecDeque::new(),
          } 
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.time.push_back(t);
        let x = self.time.binary_search(&(t - 3000)).unwrap_or_else(|i| i);
        for _ in 0..x{
            self.time.pop_front();
        }
        self.time.len() as i32
    }
}


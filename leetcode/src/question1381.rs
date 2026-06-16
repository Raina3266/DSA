struct CustomStack {
    stack: Vec<i32>,
    top: i32,
}


impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack {
            stack: vec![0; maxSize as usize],
            top: -1,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.top < (self.stack.len() as i32 - 1) {
            self.top += 1;
            self.stack[self.top as usize] = x;
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.top != -1 {
            let value = self.stack[self.top as usize];
            self.top -= 1;
            return value;
        }
        -1
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..std::cmp::min(k, self.top + 1) {
            self.stack[i as usize] += val;
        }
    }
}


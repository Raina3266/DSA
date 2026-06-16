struct MyCircularQueue {
    data: Vec<i32>,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self { data: Vec::with_capacity(k as usize) }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.data.push(value);
            return true;
        }
        false
    }

    fn de_queue(&mut self) -> bool {
        if !self.is_empty() {
            self.data.remove(0);
            return true;
        }
        false
    }

    fn front(&self) -> i32 {
        *self.data.first().unwrap_or(&-1)
    }

    fn rear(&self) -> i32 {
        *self.data.last().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn is_full(&self) -> bool {
        self.data.len() == self.data.capacity()
    }
}

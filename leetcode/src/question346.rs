use std::collections::VecDeque;

struct MovingAverage {
    stream: VecDeque<i32>,
    size: i32,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            stream: VecDeque::new(),
            size,
        }
    }

    fn next(&mut self, val: i32) -> f64 {

        self.stream.push_back(val);
        let length = (self.stream.len()) as i32;
        if length <= self.size {
            let ans: f64 = self.stream.iter().sum::<i32>() as f64 / length as f64;
            return ans;
        } else {
            for _ in 0..=(length - self.size){
                self.stream.pop_front();
            }
            let ans: f64 = self.stream.iter().sum::<i32>() as f64 / self.size as f64;
            return ans;
        }
    }
}

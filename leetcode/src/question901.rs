
struct StockSpanner {
    stock: Vec<i32>
}

impl StockSpanner {
    fn new() -> Self {
        Self { stock: vec![] }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut ans = 0;
        if self.stock.is_empty() {
            self.stock.push(price);
            return 1
        } else {
            let mut queue = self.stock.clone();
            while !queue.is_empty() && queue.last().unwrap() <= &price {
                queue.pop().unwrap();
                ans += 1;
            }
            self.stock.push(price);
        }
        ans 
    }
}
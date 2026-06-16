struct TreeNode {
    value: i32,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl TreeNode {
    fn visit(&self, mut f: impl FnMut(&i32)) {
        f(&self.value);
        if let Some(left) = self.left.as_ref() {
            left.visit(&mut f);
        }
        if let Some(right) = self.right.as_ref() {
            right.visit(&mut f);
        }
    }
    fn sum(&self)-> i32 {
        let mut sum = 0;
        self.visit(|node| {
            sum += node;
        });
        sum
    }
    fn product(&self) -> i32 {
        let mut product = 0;
        self.visit(|node|{ 
            product *= node;
        });
        product
    }
    fn max(&self) -> i32 {
        let mut max = self.value;
        self.visit(|node| {
            max = std::cmp::max(*node, max);
        });
        max
    }
    fn min(&self) -> i32 {
        let mut min = self.value;
        self.visit(|node| {
            min = std::cmp::min(min, *node)
        });
        min
    }
}

#[test]
fn foo() {
    let mut max = 0;

    for i in [1, 4, 2, 6] {
        max = std::cmp::max(max, i);
    }

    assert_eq!(max, 6);
}

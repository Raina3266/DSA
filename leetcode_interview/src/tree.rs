use std::collections::VecDeque;

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            value: 1,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: Some(Box::new(TreeNode {
                    value: 4,
                    left: Some(Box::new(TreeNode {
                        value: 8,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(TreeNode {
                        value: 9,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(TreeNode {
                    value: 5,
                    left: Some(Box::new(TreeNode {
                        value: 10,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 3,
                left: Some(Box::new(TreeNode {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
        }
    }
    fn dfs(self) {
        let mut to_visit = Vec::new();
        to_visit.push(self);
        while let Some(pop) = to_visit.pop() {
            println!("{}", pop.value);
            if let Some(right) = pop.right {
                to_visit.push(*right)
            }
            if let Some(left) = pop.left {
                to_visit.push(*left);
            }
        }
    }
    fn bfs(self) {
        let mut to_visit = VecDeque::new();
        to_visit.push_back(self);
        while let Some(pop) = to_visit.pop_front() {
            println!("{}", pop.value);
            if let Some(left) = pop.left {
                to_visit.push_back(*left);
            }
            if let Some(right) = pop.right {
                to_visit.push_back(*right)
            }
        }
    }
    fn dfs_recursive(self) {
        println!("{}", self.value);
        if let Some(left) = self.left {
            left.dfs_recursive();
        }
        if let Some(right) = self.right {
            right.dfs_recursive();
        }
    }
}

#[test]
fn feature() {
    let tree = TreeNode::new();
    tree.dfs();
    panic!()
}

/*
Rule for "visitor pattern" (for depth/breadth-first search, but also this is quite a general rule):
- when you have a function that takes a function, always take <F> ... where F: Fn*(Input) -> Output

How to choose Fn* (in order):
- If the parameter function is only going to be called at most once (provable at compile time), take FnOnce
- if the parameter function may be called many times, but never from different threads, take FnMut
- if it may be called from different threads at the same time, take FnRule for "visitor pattern" (for depth/breadth-first search, but also this is quite a general rule):
when you have a function that takes a function, always take <F> ... where F: Fn*(Input) -> Output
How to choose Fn* (in order):
If the parameter function is only going to be called at most once (provable at compile time), take FnOnce
if the parameter function may be called many times, but never from different threads, take FnMut
if it may be called from different threads at the same time, take Fn
When do I need to add the & to it:
if it's recursive
which reference type? & if it's an Fn, &mut if it's an FnMut

When do I need to add the & to it:
- if it's recursive
- which reference type? & if it's an Fn, &mut if it's an FnMut
*/

fn dfs(tree: &TreeNode) -> i32 {
    let mut max_val = tree.value;
    let f = |x| max_val = std::cmp::max(x, max_val);
    dfs_max(tree, f);
    max_val
}

fn dfs_max(tree: &TreeNode, mut f: impl FnMut(i32)) {
    let mut to_visit = Vec::new();
    to_visit.push(tree);
    while let Some(pop) = to_visit.pop() {
        f(pop.value);
        if let Some(left) = pop.left.as_deref() {
            to_visit.push(left);
        }
        if let Some(right) = pop.right.as_deref() {
            to_visit.push(right);
        }
    }
}

use std::collections::VecDeque;

struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            value: 1,
            left: Some(Box::new(Tree {
                value: 2,
                left: Some(Box::new(Tree {
                    value: 4,
                    left: Some(Box::new(Tree {
                        value: 8,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Tree {
                        value: 9,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Tree {
                    value: 5,
                    left: Some(Box::new(Tree {
                        value: 10,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
            })),
            right: Some(Box::new(Tree {
                value: 3,
                left: Some(Box::new(Tree {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
        }
    }
}

fn invert_tree(tree: &mut Tree) {

    std::mem::swap(&mut tree.left, &mut tree.right);
    if let Some(left) = &mut tree.left {
        invert_tree(left);
    }
    if let Some(right) = tree.left.as_deref_mut() {
        invert_tree(right);
    }
}

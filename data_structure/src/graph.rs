struct CityData {
    value: i32,
}
struct RoadData {
    value: u32,
}

mod objects_and_pointers {
    use std::{cell::RefCell, rc::Weak};

    struct Node {
        data: i32,
        neighbors: Vec<Weak<RefCell<i32>>>,
    }
}

mod adjacency_list {
    use std::collections::{HashMap, HashSet};

    struct Graph {
        nodes: Vec<i32>,
        edges: HashMap<usize, HashSet<usize>>,
    }

    impl Graph {
        fn visit_starting_from(&self, index: usize, mut visit: impl FnMut(i32)) {
            let mut to_visit = 
        }
    }
}

mod matrix {
    use super::*;

    struct Graph {
        nodes: Vec<i32>,
        edges: Vec<bool>,
    }
}

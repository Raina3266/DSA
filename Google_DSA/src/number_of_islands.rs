// compare three way to represent grids and choose one
// store as bool?
// have you thought about the stack overflow risk with this recursive approach?
// less step?

use std::collections::{HashMap, HashSet};

struct Grid {
    data: Vec<Vec<char>>,
    edges: HashMap<(usize, usize), HashSet<(usize, usize)>>,
}

impl Grid {
    fn parse(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let columns = data[0].len();
        let mut edges = HashMap::new();
        for (row, line) in data.iter().enumerate() {
            for (column, node) in line.iter().enumerate() {
                if node == &'1' {
                    let mut neighbours = HashSet::new();
                    if column > 0 && data[row][column - 1] == '1' {
                        neighbours.insert((row, column - 1));
                    }
                    if column < columns - 1 && data[row][column + 1] == '1' {
                        neighbours.insert((row, column + 1));
                    }
                    if row > 0 && data[row - 1][column] == '1' {
                        neighbours.insert((row - 1, column));
                    }
                    if row < rows - 1 && data[row + 1][column] == '1' {
                        neighbours.insert((row + 1, column));
                    }
                    edges.insert((row, column), neighbours);
                }
            }
        }
        Self { data, edges }
    }
    fn num_islands(&self) -> usize {
        let mut num = 0;
        let mut visited = HashSet::new();
        for start in self.edges.keys() {
            if !visited.contains(start) {
                num += 1;
                visited.insert(*start);
                self.can_visit_from_one_node_dfs(start, &mut visited);
            }
        }
        num
    }

    fn can_visit_from_one_node_recursive(
        &self,
        start: &(usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) {
        if let Some(neighbours) = self.edges.get(start) {
            for neighbour in neighbours {
                if !visited.contains(neighbour) {
                    visited.insert(*neighbour);
                    self.can_visit_from_one_node_recursive(neighbour, visited);
                }
            }
        }
    }

    fn can_visit_from_one_node_dfs(
        &self,
        start: &(usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) {
        let mut to_visit = vec![];
        to_visit.push(start);
        while let Some(pop) = to_visit.pop() {
            if let Some(neighbours) = self.edges.get(pop) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) { 
                        to_visit.push(neighbour);
                        visited.insert(*neighbour);
                    }
                }
            }
        }
    }
}

#[test]
fn test1() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let grid = Grid::parse(grid);
    assert_eq!(grid.num_islands(), 1);
}

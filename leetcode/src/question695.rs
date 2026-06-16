use std::collections::HashSet;

struct Grid {
    data: Vec<Vec<i32>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(input: Vec<Vec<i32>>) -> Self {
        let height = input.len() as isize;
        let width = input[0].len() as isize;
        Grid {
            data: input,
            width,
            height,
        }
    }

    fn visit(
        &mut self,
        x: isize,
        y: isize,
        visited: &mut HashSet<(isize, isize)>,
        count: &mut i32,
    ) {
        if self.data[x as usize][y as usize] == 0
            || visited.contains(&(x, y))
            || x < 0
            || x >= self.height
            || y < 0
            || y >= self.width
        {
            return;
        }
        self.data[x as usize][y as usize] = 0;
        visited.insert((x, y));
        *count += 1;
        if x > 0 {
            self.visit(x - 1, y, visited, count); // Up
        }
        if x < self.height - 1 {
            self.visit(x + 1, y, visited, count); // Down
        }
        if y > 0 {
            self.visit(x, y - 1, visited, count); // Left
        }
        if y < self.width - 1 {
            self.visit(x, y + 1, visited, count); // Right
        }
    }
}

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = Grid::new(grid);
    let mut max = 0;
    let mut sett: HashSet<(isize, isize)> = HashSet::new();

    for x in 0..grid.height {
        for y in 0..grid.width {
            if grid.data[x as usize][y as usize] == 1 && !sett.contains(&(x, y)) {
                let mut count = 0;
                grid.visit(x, y, &mut sett, &mut count);
                max = std::cmp::max(max, count);
            }
        }
    }
    max
}

#[test]
fn feature() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    assert_eq!(max_area_of_island(grid), 6);
}

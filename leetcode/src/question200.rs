struct Grid {
    data: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(input: Vec<Vec<char>>) -> Self {
        let height = input.len() as isize;
        let width = input[0].len() as isize;
        Grid {
            data: input,
            width,
            height,
        }
    }

    fn visit(&mut self, x: isize, y: isize) {
        if self.data[x as usize][y as usize] == '0'
            || x < 0
            || x >= self.height
            || y < 0
            || y >= self.width
        {
            println!("{}", self.height);
            println!("{x}, {y}");
            return;
        }
        self.data[x as usize][y as usize] = '0';
        if x > 0 {
            self.visit(x - 1, y);
        }
        if x + 1 < self.height {
            self.visit(x + 1, y);
        }
        if y > 0 {
            self.visit(x, y - 1);
        }
        if y + 1 < self.width {
            self.visit(x, y + 1);
        }
    }
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = Grid::new(grid);
    let mut count = 0;
    for x in 0..grid.height {
        for y in 0..grid.width {
            if grid.data[x as usize][y as usize] == '1' {
                count += 1;
                grid.visit(x, y);
            }
        }
    }
    count
}



// vec!['1','1','1','1','0'],
// vec!['1','1','0','1','0'],
// vec!['1','1','0','0','0'],
// vec!['0','0','0','0','0']

// vec!['1','1','0','0','0'],
// vec!['1','1','0','0','0'],
// vec!['0','0','1','0','0'],
// vec!['0','0','0','1','1']

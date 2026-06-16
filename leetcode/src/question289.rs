// Game of Life
// Any live cell with fewer than two live neighbors dies as if caused by under-population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by over-population.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

struct Grid {
    data: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}
impl Grid {
    fn new(data: Vec<Vec<i32>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            width,
            height,
        }
    }
    fn live_neighbors(&self, (x, y): (usize, usize)) -> i32 {
        let mut live_count = 0;
        // down
        if y + 1 < self.height && self.data[y + 1][x] == 1 {
            live_count += 1
        }
        // up
        if y > 0 && self.data[y - 1][x] == 1 {
            live_count += 1
        }
        // right
        if x + 1 < self.width && self.data[y][x + 1] == 1 {
            live_count += 1
        }
        // left
        if x > 0 && self.data[y][x - 1] == 1 {
            live_count += 1
        }
        // upleft
        if y > 0 && x > 0 && self.data[y - 1][x - 1] == 1 {
            live_count += 1
        }
        // upright
        if y > 0 && x + 1 < self.width && self.data[y - 1][x + 1] == 1 {
            live_count += 1
        }
        // downleft
        if y + 1 < self.height && x > 0 && self.data[y + 1][x - 1] == 1 {
            live_count += 1
        }
        // downright
        if y + 1 < self.height && x + 1 < self.width && self.data[y + 1][x + 1] == 1 {
            live_count += 1
        }
        live_count
    }
}

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let grid = Grid::new(board.clone());
    let mut new_board = board.clone();
    let mut cells_need_change: Vec<(usize, usize)> = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let live_neighbors = grid.live_neighbors((x, y));
            if live_neighbors < 2 && new_board[y][x] == 1
                || live_neighbors > 3 && new_board[y][x] == 1
                || live_neighbors == 3 && new_board[y][x] == 0
            {
                cells_need_change.push((x, y));
            }
        }
    }
    for cell in cells_need_change {
        if new_board[cell.1][cell.0] == 1 {
            new_board[cell.1][cell.0] = 0;
        } else {
            new_board[cell.1][cell.0] = 1;
        }
    }
    *board = new_board;
}

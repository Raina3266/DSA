use std::{cmp::Reverse, collections::BinaryHeap};



// heights[row][col]
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let col: usize = heights[0].len().try_into().unwrap();
    let row: usize = heights.len().try_into().unwrap();
    let mut dist = vec![vec![i32::MAX; col]; row];
    let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    dist[0][0] = 0;
    min_heap.push(Reverse((0, 0, 0)));
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];


    let mut x = 0;
    let mut y = 0;
    loop {
        let effort: Reverse<(i32, usize, usize)> = min_heap.pop().unwrap();
        if effort.0.0 >= dist[x][y] {
            continue;
        }
        if x == row - 1 && y == col - 1 {
            return effort.0.0;
        }
        for (dx, dy) in directions.iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if x < row && y < col {
                let new_effort = std::cmp::max(effort.0.0, ((heights[x][y]) - heights[nx][ny]).abs());
                if new_effort < dist[nx][ny] {
                    dist[nx][ny] = new_effort;
                    x = nx;
                    y = ny;
                    min_heap.push(Reverse((new_effort, nx, ny)));
                }
            }
    }
}

}


#[test]
fn feature() {
    let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
    assert_eq!(minimum_effort_path(heights), 2)
}

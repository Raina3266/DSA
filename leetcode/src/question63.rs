// obstacleGrid = [[0,1],[0,0]]

// m down, n right
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid.is_empty() || obstacle_grid[0].is_empty() || obstacle_grid[0][0] == 1 {
        return 0;
    }
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();


    if obstacle_grid.len() == 1 && obstacle_grid[0].len() == 1 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        return 1;
    }
    if obstacle_grid.iter().all(|x| x.len() == 1) {
        if obstacle_grid.iter().any(|x| x[0] == 1) {
            return 0;
        }
        return 1;
    }
    if obstacle_grid.len() == 1 {
        if obstacle_grid[0].iter().any(|x| *x == 1) {
            return 0;
        }
        return 1;
    }

    let (head1, tail1) = obstacle_grid.split_first().unwrap();

    let mut head2 = vec![];
    let mut tail2 = vec![];
    for col in &obstacle_grid {
        head2.push(col[0]);
        tail2.push(col[1..].to_vec());
    }

        unique_paths_with_obstacles(tail1.to_vec())
        + unique_paths_with_obstacles(tail2)
}

#[test]
fn feature() {}

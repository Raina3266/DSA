

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut map_row: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut map_column: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut ans = 0;
    let num_columns = grid[0].len(); 
    let mut columns = vec![Vec::new(); num_columns];

    for row in &grid {
        *map_row.entry(row.clone()).or_insert(0) += 1;
        for (index, num) in row.iter().enumerate(){
            columns[index].push(*num);
        }
    }
    for column in columns{
        *map_column.entry(column.clone()).or_insert(0) += 1;
        
    }
    for p in map_row.iter(){
        if map_column.contains_key(p.0) {
            ans += p.1 * map_column.get(p.0).unwrap();     
        }
    }
   ans
}
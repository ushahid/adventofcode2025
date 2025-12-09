use std::fs;
use std::collections::HashMap;

fn put_ray(grid: &mut Vec<Vec<char>>, num_grid: &mut Vec<Vec<u64>>, prev_count: u64, row: usize, col: i32, num_cols: usize) {
    if col >= 0 && col < num_cols as i32 {
        grid[row][col as usize] = '|';
        num_grid[row][col as usize] += prev_count;
        
    }
}



fn count_paths(grid: &Vec<Vec<char>>, row: usize, col: usize, num_rows: usize, num_cols: usize, count: u64, seen: &mut HashMap<(usize, usize), u64>) -> u64 {
    if seen.contains_key(&(row, col)) {
        return *seen.get(&(row, col)).unwrap();
    }

    let mut c = 0;
    if row == num_rows - 1 {
        c =  count;
    } else if grid[row + 1][col] == '|' {
        c = count_paths(grid, row + 1, col, num_rows, num_cols, count, seen);
    } else {
        let left_paths = count_paths(grid, row + 1, col - 1, num_rows, num_cols, count, seen);
        let right_paths = count_paths(grid, row + 1, col + 1, num_rows, num_cols, count, seen);
        c =  left_paths + right_paths;
    }
    seen.insert((row, col), c);
    return c;
}



fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }
    grid[0]
        .iter_mut()
        .for_each(|x| {
            if *x == 'S' {
                *x = '|'
            }
        });
    let idx_start = grid[0].iter().position(|x| *x == '|').unwrap();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut num_splits = 0;
    let mut num_grid: Vec<Vec<u64>> = vec![vec![0; num_cols]; num_rows];
    num_grid[0][idx_start] = 1;
    for row in 1..num_rows {
        for col in 0..num_cols {
            if grid[row - 1][col] == '|' {
                let prev_count = num_grid[row - 1][col];
                match grid[row][col] {
                    '.' => {
                        grid[row][col] = '|';
                        num_grid[row][col] += prev_count;
                    },
                    '^' => {
                        put_ray(&mut grid, &mut num_grid, prev_count , row, col as i32 - 1, num_cols);
                        put_ray(&mut grid, &mut num_grid, prev_count, row, col as i32 + 1, num_cols);
                        num_splits += 1;
                    },
                    _ => ()
                }
            } 
        }
    }
    println!("Number of splits: {}", num_splits);
    let num_paths: u64 = num_grid[num_rows - 1].iter().sum();
    println!("Number of paths: {}", num_paths);
    // for i in 0..num_rows {
    //     for j in 0..num_cols {
    //         print!("{:0>3}  ", num_grid[i][j]);
    //     }
    //     print!("\n");
    //     // println!("{:?}", grid[i]);
    // }

    let mut seen = HashMap::new();
    let num_paths = count_paths(&grid, 0, idx_start, num_rows, num_cols, 1, &mut seen);
    println!("{}", num_paths);

} 

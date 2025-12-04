use std::fs;


fn remove_rolls(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut rolls_removed = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut num_rolls = 0;
            if grid[row][col] == '@' {
                for (delta_row, delta_col) in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let r = row as i32 + delta_row;
                    let c = col as i32 + delta_col;
                    if r >= 0 && r < grid.len() as i32 && c >= 0 && c < grid[0].len() as i32{
                        if grid[r as usize][c as usize] == '@' || grid[r as usize][c as usize] == 'x'{
                            num_rolls += 1;
                        }
                    }
                }
                if num_rolls < 4 {
                    rolls_removed += 1;
                    grid[row][col] = 'x';
                }
            }
        }
    }
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'x' {
                grid[row][col] = '.';
            }
        }
    }
    return rolls_removed;
}



fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Cannot read the input file...");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }
    let mut total_removed_rolls = 0;
    let mut rolls_removed = 1;
    let mut first_run = true;
    while rolls_removed > 0 {
        rolls_removed = remove_rolls(&mut grid);
        total_removed_rolls += rolls_removed;
        if first_run {
            println!("Part 1 answer {}", rolls_removed);
        }
        first_run = false;
    }
    println!("Part 2 answer: {}", total_removed_rolls);
}

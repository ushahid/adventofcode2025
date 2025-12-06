use core::panic;
use std::{fs, str::SplitWhitespace};
use std::time::Instant;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");

    let p1_start = Instant::now();
    let mut iterators: Vec<SplitWhitespace> = Vec::new();
    for line in data.lines().rev() {
        iterators.push(line.split_whitespace());
    }

    let mut sum: u64 = 0;
    while let Some (op) = iterators[0].next() {
        let mut add_ans: u64 = 0;
        let mut mul_ans: u64 = 1;
        for i in 1..iterators.len() {
            match op {
                "+" => add_ans += iterators[i].next().unwrap().parse::<u64>().unwrap(),
                "*" => mul_ans *= iterators[i].next().unwrap().parse::<u64>().unwrap(),
                _ => panic!("Invalid operand")
            } 
        }
        if op == "+" {
            sum += add_ans;
        } else {
            sum += mul_ans;
        }
    }
    println!("Part 1 sum {}", sum);
    println!("Part 1 time: {}", p1_start.elapsed().as_micros());


    let p2_start = Instant::now();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }

    let mut sum = 0;
    let mut op = ' ';
    let mut mul_result = 1;
    let mut sum_result = 0;

    for col in 0..grid[0].len() {
        let mut num_str: String  = "".to_string();
        for row in 0..(grid.len() - 1){
            if grid[row][col] != ' ' {
                num_str.push(grid[row][col]);
            }
        }
        if grid[grid.len() - 1][col] != ' ' {
            op = grid[grid.len() - 1][col];
        }
        if num_str != "" {
            let num = num_str.parse::<u64>().unwrap();
            match op {
                '+' => sum_result += num,
                '*' => mul_result *= num,
                _ => panic!("Invalid operand {}", op)
            }
        } else {
            match op {
                '+' => sum += sum_result,
                '*' => sum += mul_result,
                _ => panic!("Invalid operand {}", op)
            }
            mul_result = 1;
            sum_result = 0;
        }
        
    }
    match op {
        '+' => sum += sum_result,
        '*' => sum *= mul_result,
        _ => panic!("Invalid operand {}", op)
    }
    println!("Part 2 sum {}", sum);
    println!("Part 2 time: {}", p2_start.elapsed().as_micros());

}

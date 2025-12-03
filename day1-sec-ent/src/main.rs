use std::fs;




fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Cannot read the input file...");

    let mut current = 50;
    let mut passed_zero_count = 0;
    let mut zero_count = 0;
    for line in data.lines() {
        let data: Vec<char> = line.chars().collect();
        let num: i32 = data[1..].into_iter().collect::<String>().parse().expect("Invalid input");
        let mut delta: i32 = -1;
        if data[0] == 'R' {
            delta = 1;
        }
        for _ in 0..num {
            current += delta;
            if current == 100 {
                current = 0;
            } else if current == -1 {
                current = 99;
            }
            if current == 0 {
                passed_zero_count += 1;
            }
        }
        if current == 0 {
            zero_count += 1;
        }
    }
    println!("Zero count {}", zero_count);
    println!("Passed zero count {}", passed_zero_count);
}

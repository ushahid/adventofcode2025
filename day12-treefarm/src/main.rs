use std::fs;
use regex::Regex;


fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let pattern = Regex::new(r"(?<width>[0-9]+)x(?<height>[0-9]+): (?<presents>[0-9 ]+)").unwrap();

    let mut total = 0;
    for x in  pattern.captures_iter(&data) {
        let width = x.name("width").unwrap().as_str().parse::<u64>().unwrap();
        let height = x.name("height").unwrap().as_str().parse::<u64>().unwrap();
        let presents: Vec<u64> = x.name("presents").unwrap().as_str().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        if (width * height) >= 9 * presents.iter().sum::<u64>() {
            total += 1;
        }
    }
    println!("Part 1 answer: {}", total);

}

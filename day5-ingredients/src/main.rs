use std::{cmp::Ordering, fs};



fn merge_and_sort_ranges(ranges: &mut Vec<(u64, u64)>) {
    ranges.sort_by(|a, b| {a.0.cmp(&b.0)});
    let mut i = 1;
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;
    while i < ranges.len() {
        if current_end < ranges[i].0 {
            merged_ranges.push((current_start, current_end));
            current_start = ranges[i].0;
            current_end = ranges[i].1;
        } else if current_end <= ranges[i].1{
            current_end = ranges[i].1;
        }
        i += 1;
    }
    merged_ranges.push((current_start, current_end));
    *ranges = merged_ranges;
}


fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Could not read the input file");
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = data[0].lines().map(|line| {
        let nums: Vec<&str> = line.split("-").collect();
        (nums[0].parse::<u64>().unwrap(), nums[1].parse::<u64>().unwrap())
    }).collect();
    merge_and_sort_ranges(&mut ranges);

    let mut num_ids = 0_u64;
    for id in data[1].lines() {
        let num_id = id.parse::<u64>().unwrap();
        let res  = ranges.binary_search_by(|&(start, end)| {
            if num_id >= start && num_id <= end {
                Ordering::Equal
            } else if start > num_id {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        if res.is_ok() {
            num_ids += 1;
        }
    }
    println!("Fresh ingredients: {}", num_ids);

    let mut total_ids = 0_u128;
    for range in ranges.iter() {
        total_ids += range.1 as u128 - range.0 as u128 + 1_u128
    }
    println!("Total ids: {}", total_ids);

}

use std::fs;
use std::cmp::Ordering;



fn get_jolts(vec: &Vec<u64>, num_batteries: usize) -> u64{
    let mut num: u64 = 0;
    let mut max_idx: usize = 0;
    for i in (0..num_batteries).rev() {
        let (idx, max) = vec[max_idx..vec.len() - i]
                                        .iter()
                                        .enumerate()
                                        .max_by(|(left_idx, left), (right_idx, right)| {
                                            let val_cmp = left.cmp(right);
                                            match val_cmp {
                                                Ordering::Less => Ordering::Less,
                                                Ordering::Greater => Ordering::Greater,
                                                Ordering::Equal => left_idx.cmp(right_idx).reverse()
                                            }
                                        }).unwrap();
        num += max * 10_u64.pow(i as u32);
        max_idx = max_idx + idx + 1;
    }
    return num;
}


fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Cannot read the input file...");
    let mut sum = 0;
    for line in data.lines() {
        let vec: Vec<u64> = line.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
        sum += get_jolts(&vec, 2);
    }
    println!("Part 1 answer: {}", sum);
    
    sum = 0;
    for line in data.lines() {
        let vec: Vec<u64> = line.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
        sum += get_jolts(&vec, 12);
    }
    println!("Part 2 answer: {}", sum);
}

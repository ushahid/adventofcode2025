use std::fs;
use std::collections::HashSet;

fn is_even_len(s: &String) -> bool{
    return s.len() % 2 == 0;
}


fn to_u64(s: &String) -> u64 {
    return s.parse::<u64>().expect("Could not parse the numerical value!");
}


fn to_str(num: &u64) -> String {
    return num.to_string();
}


fn get_next_even_len(s: &String) -> String {
    let mut new_str = "1".to_string();
    new_str += "0".repeat(s.len()).as_str();
    return new_str
}


// invalid sum for part 2
fn get_invalid_range_sum(start: &String, end: &String) -> u64 {
    let max_digits = (start.len() / 2).max(end.len() / 2);
    let num_start = to_u64(&start);
    let num_end = to_u64(&end);
    let mut seen = HashSet::new();
    let mut invalid_sum = 0;
    for digit in 0..max_digits {
        let min_rep = ((start.len() as f64 / (digit + 1) as f64)
                                .floor() as u64).max(2);
        let max_rep = (end.len() as f64 / (digit + 1) as f64)
                                .floor() as u64;
        // println!("{}, {} {}, {}-{}", digit + 1, min_rep, max_rep, start.len(), end.len())
        let start_num = (10 as u64).pow(digit as u32);
        let end_num = (10 as u64).pow((digit + 1) as u32);
        for num_to_repeat in start_num..end_num { 
            for num_rep in min_rep..=max_rep{
                let num = to_u64(&num_to_repeat.to_string().repeat(num_rep as usize));
                if num >= num_start && num <= num_end
                                    && !seen.contains(&num){
                    invalid_sum += num;
                    seen.insert(num);
                }
            }
        }
    }
    return invalid_sum;
}


fn main() {
    let data: String = fs::read_to_string("input.txt").expect("Cannot read the input file...");
    // let mut invalid = 0;
    let mut invalid_p2 = 0;
    for range_str in data.split(",") {
        // println!("======= {} =======", range_str);
        let range: Vec<String> = range_str.split("-").map(|s| s.to_string()).collect();
        invalid_p2 += get_invalid_range_sum(&range[0], &range[1]);
        // let mut start = range[0].clone();
        // if !is_even_len(&start) {
        //     start = get_next_even_len(&range[0]);
        //     if to_u64(&start) > to_u64(&range[1]) {
        //         continue;
        //     }
        // }
        // let (first_half, _) = start.split_at(start.len() / 2);
        // let mut num = to_u64(&first_half.to_string());
        // let end = &range[1];
        // let mut doubled = to_u64(&first_half.repeat(2));
        // while doubled <  to_u64(end){
        //     doubled = to_u64(&to_str(&num).repeat(2));
        //     if doubled >= to_u64(&start) && doubled <= to_u64(&end) {
        //         // println!("{}", doubled);
        //         invalid += doubled;
        //     }
        //     num += 1;
        // }
    }
    // println!("Invalid sum {}", invalid);
    println!("Invalid sum P2 {}", invalid_p2);
}

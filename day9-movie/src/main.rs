use std::fs;
// use geo::Contains;
// use geo::{LineString, Point, Polygon, point};
use geo::{Contains};
use geo::{LineString, Polygon};
use itertools::Itertools;



fn calculate_area(p1: &(f64, f64), p2: &(f64, f64)) -> u64 {
    return (((p1.0 - p2.0).abs() + 1.) * ((p1.1 - p2.1).abs() + 1.)) as u64;
}



fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let mut tiles= Vec::new();
    for line in data.lines() {
        let elements: Vec<&str> = line.split(",").collect();
        tiles.push((elements[0].parse::<f64>().unwrap(), elements[1].parse::<f64>().unwrap()));
    }

    let ls = LineString::from(tiles.clone());
    let poly = Polygon::new(ls, vec![]);

    let mut max_area = 0;
    for comb in tiles.iter().combinations(2) {
        let corners = vec![
                comb[0].clone(),
                (comb[1].0, comb[0].1),
                comb[1].clone(),
                (comb[0].0, comb[1].1),
        ];

        let rectangle = Polygon::new(LineString::from(corners), vec![]);
        let is_valid = poly.contains(&rectangle);
        
        if is_valid {
            let area = calculate_area(&comb[0], &comb[1]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Max area is {}", max_area);
}

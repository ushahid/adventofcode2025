use std::{collections::HashMap, fs};
use keyed_priority_queue::KeyedPriorityQueue;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::time::Instant;



#[derive(Debug, Clone)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64
}


fn distance(left: &Vec3d, right: &Vec3d) -> f64{
    return ((left.x - right.x).powf(2.0) + (left.y - right.y).powf(2.0) + (left.z - right.z).powf(2.0)).sqrt()
}


fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let mut junctions = Vec::new();
    let timer = Instant::now();

    for line in data.lines() {
        let mut split = line.split(',');
        let vec3d = Vec3d {
            x: split.next().unwrap().parse::<f64>().unwrap(),
            y: split.next().unwrap().parse::<f64>().unwrap(),
            z: split.next().unwrap().parse::<f64>().unwrap(),
        };
        junctions.push(vec3d)
    }

    let mut queue = KeyedPriorityQueue::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            queue.push((i, j), Reverse(OrderedFloat(distance(&junctions[i], &junctions[j]))));
        }
    }
    
    let mut circuits: Vec<usize> = (0..junctions.len()).collect();
    let mut num_connections = 0;
    while let Some(((i, j), Reverse(_d))) = queue.pop() {
        if circuits[i] != circuits[j] {
            let new_idx;
            let old_idx;
            if circuits[i] < circuits[j] {
                new_idx = circuits[i];
                old_idx = circuits[j];
            } else {
                new_idx = circuits[j];
                old_idx = circuits[i];
            }
            for k in 0..circuits.len() {
                if circuits[k] == old_idx {
                    circuits[k] = new_idx;
                }
            }
            if is_last(&circuits) {
                println!("Part 2 answer: {}", junctions[i].x * junctions[j].x);
                println!("Part 2 time: {} ms", timer.elapsed().as_millis());
                break;
            }
        }
        num_connections += 1;
        if num_connections == 1000 {
            let mut counts = HashMap::new();
            for i in 0..circuits.len() {
                if !counts.contains_key(&circuits[i]) {
                    counts.insert(circuits[i], 1);
                } else {
                    (*counts.get_mut(&circuits[i]).unwrap()) += 1;
                }
            }
            let mut sort_vals: Vec<i32> = counts.values().cloned().collect();
            sort_vals.sort_by_key(|&w| Reverse(w));
            println!("Part 1 answer: {}", sort_vals[0] * sort_vals[1] * sort_vals[2]);
            println!("Part 1 time: {} ms", timer.elapsed().as_millis());
        }
    }
}


fn is_last(circuits: &Vec<usize>) -> bool {
    for i in 0..circuits.len() {
        if circuits[i] != 0 {
            return false;
        }
    }
    return true;
}
 
use std::fs;
use std::collections::{HashSet, VecDeque};
use regex::Regex;
use good_lp::{Expression, Solution, SolverModel, Variable, constraint, default_solver, variable, variables};



fn switches_to_u64(switches: &Vec<u64>) -> u64 {
    let mut bin_switches: u64 = 0;
    for switch in switches {
        let to_or: u64 = 1 << switch;
        bin_switches |= to_or;
    }
    return bin_switches;
}


fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let switches_regex = Regex::new(r"\((?<switches>([0-9]+,?)+)\)").unwrap();
    let target_regex = Regex::new(r"\[(?<target>[\.#]+)\]").unwrap();
    let joltage_regex = Regex::new(r"\{(?<joltage>([0-9]+,?)+)\}").unwrap();

    let mut all_targets: Vec<Vec<char>> = Vec::new();
    let mut all_switches: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut all_joltages: Vec<Vec<u64>> = Vec::new();

    for line in data.lines() {
        let Some(c) = target_regex.captures(line) else { panic!("Parsing error for Target"); };
        let target: Vec<char> = c.name("target").unwrap().as_str().chars().collect();
        all_targets.push(target);
        let mut switches: Vec<Vec<u64>> = Vec::new();
        for s in switches_regex.captures_iter(line) {
            switches.push(s.name("switches").unwrap().as_str().split(",").map(|x| x.parse::<u64>().unwrap()).collect());
        }
        all_switches.push(switches);
        let Some(c) = joltage_regex.captures(line) else { panic!("Parsing error for Joltage"); };
        let joltage: Vec<u64> = c.name("joltage").unwrap().as_str().split(",").map(|x| x.parse::<u64>().unwrap()).collect();
        all_joltages.push(joltage)
    }

    let bin_targets: Vec<u64> = all_targets
                                    .iter()
                                    .map(|c: &Vec<char>| c.iter()
                                                            .enumerate()
                                                            .filter(|(_, c)| **c == '#')
                                                            .map(|(index, _)| index as u64)
                                                            .collect::<Vec<u64>>())
                                    .map(|switches| switches_to_u64(&switches))
                                    .collect();
    let bin_switches: Vec<Vec<u64>> = all_switches
                                        .iter()
                                        .map(|x| {
                                            x.iter().map(|switches| switches_to_u64(switches)).collect()
                                        }).collect();
    
    // Part 1 solution
    let mut total = 0;
    for i in 0..bin_targets.len() {
        let mut seen: HashSet<u64> = HashSet::new();
        let mut queue: VecDeque<(u64, u64)> = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((state, depth)) = queue.pop_front() {
            if state == bin_targets[i] {
                    total += depth;
                    break;
            }
            if !seen.contains(&state) {
                seen.insert(state);
                for switch in bin_switches[i].iter() {
                    let new_state = (*switch) ^ state;
                    queue.push_back((new_state, depth + 1));
                }
            }
        }
    }
    println!("Part 1 answer: {}", total);

    for i in 0..all_joltages.len() {
        let mut vars_vec: Vec<Variable> = Vec::new();
        let mut vars = variables!();
        let mut constraints = Vec::new();
        // Add number of times each switch is pressed as a variable
        for (idx, joltage) in all_joltages[i].iter().enumerate() {
            vars_vec.push(vars.add(variable().integer().min(0)));

            let mut sum: Expression = 0.into();
            // Go through each switch combination and add as needed
            for (switch_idx, switches) in all_switches[i].iter().enumerate() {
                if switches.contains(&(idx as u64)) {
                    sum += vars_vec[switch_idx];
                }
            }
            constraints.push(constraint!(sum == ((*joltage) as u32)));
        }
        let mut total_presses: Expression = 0.into();
        for var in vars_vec.iter() {
            total_presses += var
        }
        let solution = vars.minimise(total_presses).using(default_solver).with_all(constraints).solve().unwrap();
        let mut total_presses: u64 = 0;
        for var in vars_vec.iter() {
            total_presses += solution.value(*var) as u64;
        }
        println!("Solution is {}", total_presses);
    }
}

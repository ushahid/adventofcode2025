use std::{fs, vec};
use std::collections::{HashMap, LinkedList};
use topo_sort::{TopoSort, SortResults};


fn get_count(root: &String, dest: &String, graph: &HashMap<String, LinkedList<String>>) -> u64 {
    let mut counts = vec![0; graph.len() + 1];
    let mut topo_sort = TopoSort::with_capacity(graph.len());

    let mut inv_graph: HashMap<String, LinkedList<String>> = HashMap::new();
    for (node, nbrs) in graph.iter() {
        if !inv_graph.contains_key(node) {
                inv_graph.insert(node.clone(), LinkedList::new());
        }
        for nbr in nbrs {
            if !inv_graph.contains_key(nbr) {
                inv_graph.insert(nbr.clone(), LinkedList::new());
            }
            inv_graph.get_mut(nbr).unwrap().push_back(node.clone());
        }
    }

    for (node, neighbors) in inv_graph.iter() {
        let nbrs: Vec<&String> = neighbors.iter().collect();
        topo_sort.insert(node, nbrs);
    }

    match topo_sort.into_vec_nodes() {
        SortResults::Full(nodes) => {
            let root_idx = nodes.iter().position(|&x| x == root).unwrap();
            let dest_idx = nodes.iter().position(|&x| x == dest).unwrap();
            for (idx, &node) in nodes.iter().enumerate() {
                if idx < root_idx {
                    counts[idx] = 0;
                }
                if idx == root_idx {
                    counts[idx] = 1;
                }
                for nbr in inv_graph.get(node).unwrap() {
                    let nbr_idx = nodes.iter().position(|&x| x == nbr).unwrap();
                    counts[idx] += counts[nbr_idx]
                }
            }
            return counts[dest_idx];
        },
        SortResults::Partial(_) => panic!("unexpected cycle!"),
    }
}




fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file...");
    let mut graph: HashMap<String, LinkedList<String>> = HashMap::new();
    for line in data.lines() {
        let nodes: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
        graph.insert(nodes[0][..nodes[0].len() - 1].to_string(), LinkedList::from_iter(nodes[1..].iter().map(|x| x.clone())));
    }

    let count = get_count(&"you".to_string(), &"out".to_string(), &graph);
    println!("Part 1 answer: {}", count);

    let svr_fft = get_count(&"svr".to_string(), &"fft".to_string(), &graph);
    let fft_dac = get_count(&"fft".to_string(), &"dac".to_string(), &graph);
    let dac_out = get_count(&"dac".to_string(), &"out".to_string(), &graph);
    println!("Part 2 answer: {}", svr_fft * fft_dac * dac_out);
}
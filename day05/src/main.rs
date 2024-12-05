mod edge_set;
use edge_set::EdgeSet;
use std::collections::HashSet;
use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;

fn main() {
    let mut file_name: String = String::new();
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        file_name = args[1].clone();
    } else {
        print!("Enter file name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");
        file_name = file_name.trim().to_string();
    }

    let (edges, sequences) = read_from_file(&file_name).expect("Error reading from file");

    let g: EdgeSet = EdgeSet::new_from_edges(edges);
    let mut sum: i32 = 0;
    let mut corrected_sum: i32 = 0;
    for seq in sequences {
        if validate_sequence(&g, &seq) {
            /*
            If the sequence is valid, we add the middle element to the sum.
             */
            sum += get_mid_seq_element(&seq);
        } else {
            /* 
            If the sequence is invalid, we try to correct it by building a subgraph
            from the original graph that only contains the nodes in the sequence.
            We then perform a topological sort on the subgraph which will give us a
            valid sequence if one exists. We then validate the corrected sequence
            and add the middle element to the corrected sum if it is valid.
            */ 
            let subgraph = build_subgraph(&g, &seq);
            let corrected_seq: Vec<i32> = topo_sort(&subgraph);
            if validate_sequence(&g, &corrected_seq) {
                corrected_sum += get_mid_seq_element(&corrected_seq);
            }
        }
    }

    println!("Total Sum: {}", sum);
    println!("Corrected Sum: {}", corrected_sum);
}

fn read_from_file(file_name: &str) -> io::Result<(Vec<Vec<i32>>, Vec<Vec<i32>>)> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut graph_edges: Vec<Vec<i32>> = Vec::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    let re = Regex::new(r"^(\d+\|\d+)|(\d+(?:,\s?\d+)+)$").expect("Invalid regex");
    for line in lines {
        if let Some(cap) = re.captures(&line.unwrap()) {
            if let Some(m) = cap.get(1) {
                let s = m.as_str();
                let v: Vec<i32> = parse_istring_to_vec(s, '|');
                graph_edges.push(v);
            } else if let Some(m) = cap.get(2) {
                let s = m.as_str();
                let v: Vec<i32> = parse_istring_to_vec(s, ',');
                sequences.push(v);
            }
        }
    }

    return Ok((graph_edges, sequences));
}

fn parse_istring_to_vec(s: &str, del: char) -> Vec<i32> {
    let v: Vec<i32> = s
        .split(del)
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();
    return v;
}

fn validate_sequence(g: &EdgeSet, seq: &Vec<i32>) -> bool {
    for i in seq.windows(2) {
        let src = i[0];
        let dest = i[1];
        if !g.has_edge(src, dest) {
            return false;
        }
    }
    return true;
}

fn get_mid_seq_element(seq: &Vec<i32>) -> i32 {
    let len = seq.len();
    if len % 2 == 0 {
        return seq[len / 2 - 1];
    } else {
        return seq[len / 2];
    }
}

fn build_subgraph(g: &EdgeSet, seq: &Vec<i32>) -> EdgeSet {
    let mut subgraph= EdgeSet::new();
    for &node in seq {
        subgraph.edges.entry(node).or_insert(HashSet::new());
        if let Some(edges) = g.edges.get(&node) {
            for &neighbor in edges {
                if seq.contains(&neighbor) {
                    subgraph.add_edge(node, neighbor);
                }
            }
        }
    }
    return subgraph;
}

fn topo_sort(g: &EdgeSet) -> Vec<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut stack: Vec<i32> = Vec::new();
    for node in g.edges.keys() {
        if !visited.contains(node) {
            dfs(g, *node, &mut visited, &mut stack);
        }
    }
    stack.reverse();
    return stack;
}

fn dfs(g: &EdgeSet, node: i32, visited: &mut HashSet<i32>, stack: &mut Vec<i32>) {
    visited.insert(node);
    if let Some(edges) = g.edges.get(&node) {
        for &neighbor in edges {
            if !visited.contains(&neighbor) {
                dfs(g, neighbor, visited, stack);
            }
        }
    }
    stack.push(node);
}
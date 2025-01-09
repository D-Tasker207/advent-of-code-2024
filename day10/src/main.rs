mod graph;

use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashSet;

use graph::Graph;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

    let trail_map = read_from_file(&file_name)
        .expect("Error reading from file");
    let graph = parse_map_to_graph(trail_map);
    let base_score = test_all_trailheads(&graph, false);
    println!("Single Path Score: {}", base_score);
    let all_score = test_all_trailheads(&graph, true);
    println!("All Paths Score: {}", all_score);

}

fn read_from_file(file_name: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut trail_map = Vec::new();
    for line in lines {
        trail_map.push(line
            .unwrap()
            .chars()
            .map(|v| v.to_digit(10).unwrap() as i32)
            .collect());
    }

    Ok(trail_map)
}

fn parse_map_to_graph(trail_map: Vec<Vec<i32>>) -> Graph {
    let mut graph = Graph::new();
    for (r, row) in trail_map.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            let position = (r as i32, c as i32);
            let mut edges = HashSet::new();
            for (dr, dc) in DIRECTIONS.iter() {
                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;
                if new_r >= 0 && new_r < trail_map.len() as i32
                    && new_c >= 0 && new_c < row.len() as i32 
                    && trail_map[new_r as usize][new_c as usize] == 1 + value {
                    edges.insert((new_r, new_c));
                }
            }
            graph.add_node(position, value, edges);
        }
    }
    graph
}

fn test_all_trailheads(graph: &Graph, find_all: bool) -> i32 {
    let mut total_score = 0;
    let dst_posns = graph.get_posn_by_value(9).expect("No Destinations found");
    let trailheads = graph.get_posn_by_value(0).expect("No Trailheads found");

    for trailhead_posn in trailheads.iter() {
        let trail_score = test_trailhead(graph, *trailhead_posn, &dst_posns, find_all);
        total_score += trail_score;
    }

    total_score
}

fn test_trailhead(graph: &Graph, trailhead_posn: (i32, i32), dst_posns: &HashSet<(i32, i32)>, find_all: bool) -> i32 {
    let mut score = 0; 
    for dst_posn in dst_posns.into_iter() {
        let paths = find_paths(&graph, trailhead_posn, *dst_posn, find_all);
        score += paths.len() as i32;
    }

    score
}

fn find_paths(graph: &Graph, src: (i32, i32), dst: (i32, i32), find_all: bool) -> Vec<Vec<(i32, i32)>> {
    let mut all_paths = Vec::new();
    let mut current_path = Vec::new();
    let mut visited = HashSet::new();

    dfs(graph, src, dst, &mut current_path, &mut all_paths, &mut visited, find_all);
    all_paths
}

fn dfs(graph: &Graph,
       current: (i32, i32),
       dst: (i32, i32),
       current_path: &mut Vec<(i32, i32)>,
       all_paths: &mut Vec<Vec<(i32, i32)>>,
       visited: &mut HashSet<(i32, i32)>,
       find_all: bool) -> bool {
    visited.insert(current);
    current_path.push(current);

    if current == dst {
        all_paths.push(current_path.clone());
        visited.remove(&current);
        current_path.pop();
        return !find_all;
    }

    if let Some(neighbors) = graph.get_node_by_posn(current) {
        for &neighbor_posn in neighbors.edges.iter() {
            if !visited.contains(&neighbor_posn) {
                if dfs(graph, neighbor_posn, dst, current_path, all_paths, visited, find_all) {
                        return true;
                }
            }
        }
    }
    current_path.pop();
    visited.remove(&current);
    false
}
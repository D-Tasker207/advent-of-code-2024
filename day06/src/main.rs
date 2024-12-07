use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::{HashSet, HashMap};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Position = (i32, i32);
type DirectionMap = HashMap<usize, Position>;
type Graph = HashMap<Position, DirectionMap>;

fn main() {
    // Read the file name from the command line arguments or prompt the user for the file name
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

    // Read the map and starting position from the file
    let (map, start_pos): (Vec<Vec<char>>, (i32, i32)) = read_from_file(&file_name)
        .expect("Error reading from file");

    let graph = build_graph(&map);
    let start_state = (start_pos.0, start_pos.1);
    let path = get_base_path(&graph, start_state, 0);
    let steps = path
        .iter()
        .map(|(r, c)| (r, c))
        .collect::<HashSet<_>>().len();

    println!("Number of steps: {}", steps);
    let num_loops = try_obstacles(&graph, start_state, 0, &path);
    println!("Number of loops: {}", num_loops);
}

fn read_from_file(file_name: &str) -> io::Result<(Vec<Vec<char>>, (i32, i32))> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut start_pos = (0, 0);
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        if row.contains(&'^') {
            for (i, &c) in row.iter().enumerate() {
                if c == '^' {
                    start_pos = (map.len() as i32, i as i32);
                    break;
                }
            }
        }
        map.push(row);
    }

    Ok((map, start_pos))
}

fn build_graph(map: &Vec<Vec<char>>) -> Graph {
    let nrows = map.len();
    let ncols = map[0].len();
    let mut graph: Graph = HashMap::new();

    for r in 0..nrows {
        for c in 0..ncols {
            if map[r][c] == '#' {
                continue;
            }
            let mut dir_map: DirectionMap = HashMap::new();
        
            for d in 0..4 {
                let (dr, dc) = DIRECTIONS[d];
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr < 0 || nr >= nrows as i32 || nc < 0 || nc >= ncols as i32 {
                    // Out of bounds, connect node to (nrows+1, ncols+1) to indicate out of bounds:
                    dir_map.insert(d, (-1, -1));
                } else if map[nr as usize][nc as usize] != '#' {
                    // Direction is clear, add edge
                    dir_map.insert(d,(nr, nc));
                }
            }
            if !dir_map.is_empty() {
                graph.insert((r as i32, c as i32), dir_map);
            }
        }
    }
    return graph;
}

fn get_base_path(graph: &Graph, start: Position, start_dir: usize) -> Vec<Position> {
    let mut path: Vec<(i32, i32)> = vec![start];
    let mut cur: (i32, i32) = start;
    let mut dir: usize = start_dir;
    while let Some(edges) = graph.get(&cur) {
        let next_node = edges.get(&dir);
        match next_node {
            Some(&next_node) => {
                if next_node == (-1, -1) {
                    return path
                }
                path.push(next_node);
                cur = next_node;
            }
            None => {
                dir = (dir + 1) % 4;
            }
        }
    }
    return path;
}

fn check_loop(graph: &Graph, start: Position, start_dir: usize) -> bool {
    let mut visited: HashMap<Position, Vec<usize>> = HashMap::new();
    let mut cur: Position = start;
    let mut dir: usize = start_dir;
    while let Some(edges) = graph.get(&cur) {
        let next_node = edges.get(&dir);
        match next_node {
            Some(&next_node) => {
                if next_node == (-1, -1) {
                    return false
                }
                if visited.contains_key(&next_node) &&
                    visited.get(&next_node).unwrap().contains(&dir)
                    {
                    return true;
                }
                visited.entry(next_node).or_insert(Vec::new()).push(dir);
                cur = next_node;
            }
            None => {
                dir = (dir + 1) % 4;
            }
        }
    }
    return false;
}

fn make_wall(graph: &Graph, position: Position) -> Graph {
    let mut new_graph = graph.clone();

    let affected_states: Vec<Position> = new_graph
        .iter()
        .filter_map(|(state, edges)| {
            if edges.values().any(|&(r, c)| r == position.0 && c == position.1) {
                Some(*state)
            } else {
                None
            }
        })
        .collect();

    for &st in &affected_states {
        if let Some(edges) = new_graph.get_mut(&st) {
            edges.retain(|&_, &mut (r, c)| r != position.0 || c != position.1);
        } 
    }
    return new_graph;
}

fn try_obstacles(graph: &Graph, start: Position, start_dir: usize, path: &Vec<Position>) -> i32 {
    let mut new_walls: Vec<Position> = Vec::new();
    let mut num_loops = 0;
    for &pos in path[2..].iter() {
        if pos.0 == -1 && pos.1 == -1 {
            continue;
        }
        
        let mut new_graph = graph.clone();
        new_graph = make_wall(&new_graph, pos);
        if check_loop(&new_graph, start, start_dir) && 
            !new_walls.contains(&pos) {
            new_walls.push(pos);
            num_loops += 1;
        }
    }
    return num_loops;
}

// num loops: 1623 < x < 1902
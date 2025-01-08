use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashSet;

struct Antenna {
    pos: (usize, usize),
    symbol: char,
}

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

    let map: Vec<Vec<char>> = read_from_file(&file_name)
        .expect("Error reading from file");

    let antennae = find_antennae(&map);
    let antinodes = find_antinodes(&antennae, &map);
    println!("Num of antinodes: {}", antinodes.len());

    let stepped_antinodes = find_stepped_anitnodes(&antennae, &map);
    println!("Num of stepped antinodes: {}", stepped_antinodes.len());
}

fn read_from_file(file_name: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.unwrap()
            .chars()
            .collect());
    }

    Ok(map)
}

fn find_antennae(map: &Vec<Vec<char>>) -> Vec<Antenna> {
    let mut antennae: Vec<Antenna> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennae.push(Antenna {
                    pos: (i, j),
                    symbol: cell,
                });
            }
        }
    }
    antennae
}

fn check_inbounds(pos: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    pos.0 >= 0 && pos.0 < map.len() as i32 && 
    pos.1 >= 0 && pos.1 < map[pos.0 as usize].len() as i32
}

fn step_pos_inbounds(pos: (usize, usize), dir: (i32, i32), map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let (x, y) = pos;
    let (dx, dy) = dir;
    let new_pos = (x as i32 - dx, y as i32 - dy);
    if check_inbounds(new_pos, map) {
        return Some((new_pos.0 as usize, new_pos.1 as usize));
    }
    None
}

fn find_dir(pos: (usize, usize), other_pos: (usize, usize)) -> (i32, i32) {
    (
        (other_pos.0 as i32 - pos.0 as i32),
        (other_pos.1 as i32 - pos.1 as i32),
    )
}

fn find_antinodes(antennae: &Vec<Antenna>, map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    antennae.iter()
        .flat_map(|antenna| {
            antennae.iter()
                .filter(|&a| a.symbol == antenna.symbol && a.pos != antenna.pos)
                .filter_map(move |other_antenna| 
                    step_pos_inbounds(
                        antenna.pos,
                        find_dir(antenna.pos, other_antenna.pos),
                        map))
        })
        .collect()
}

fn find_stepped_anitnodes(antennae: &Vec<Antenna>, map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    antennae.iter()
        .flat_map(|antenna| {
            antennae.iter()
                .filter(|&a| a.symbol == antenna.symbol && a.pos != antenna.pos)
                .flat_map(move |other_antenna| {
                    let dir = find_dir(antenna.pos, other_antenna.pos);

                    std::iter::successors(Some(antenna.pos), move |&pos| {
                        let next_pos = (
                            pos.0 as i32 + dir.0,
                            pos.1 as i32 + dir.1,
                        );
                        if check_inbounds(next_pos, map) {
                            Some((next_pos.0 as usize, next_pos.1 as usize))
                        } else {
                            None
                        }
                    })
                    .skip(1)  
                })
        })
        .collect()
}
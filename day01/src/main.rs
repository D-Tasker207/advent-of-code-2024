use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    print!("Enter file name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");
    file_name = file_name.trim().to_string();

    let (a, b) = read_from_file(&file_name).expect("Error reading from file");

    let min_diff = find_min_diff(&a, &b);
    println!("Min diff: {}", min_diff);

    let sim_score = find_similarity_score(&a, &b);
    println!("Similarity score: {}", sim_score);
}

fn read_from_file(file_name: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in lines {
        let nums: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        a.push(nums[0]);
        b.push(nums[1]);
    }

    return Ok((a, b));
}

fn find_min_diff(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut a = a.clone();
    let mut b = b.clone();

    a.sort();
    b.sort();

    return a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
}

fn find_similarity_score(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();
    b.iter().for_each(|i| {
        *freq_map.entry(i).or_insert(0) += 1;
    });

    return a
        .iter()
        .map(|x| {
            x * match freq_map.get(x) {
                Some(&y) => y,
                None => 0,
            }
        })
        .sum();
}

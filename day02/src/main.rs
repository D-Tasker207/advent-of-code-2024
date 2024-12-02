use std::fs::File;
use std::io::{self, BufRead, Write};

trait Betweeen {
    fn between(&self, a: i32, b: i32) -> bool;
}
impl Betweeen for i32 {
    fn between(&self, a: i32, b: i32) -> bool {
        return *self >= a && *self <= b;
    }
}

fn main() {
    print!("Enter file name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)
        .expect("Failed to read line");
    file_name = file_name.trim().to_string();

    let data = read_from_file(&file_name)
        .expect("Error reading from file");

    let count = classify_safety_records(&data);
    println!("Num Safe Records: {}", count);

    let count = classify_with_problem_dampener(&data);
    println!("Num Safe Records with Dampener: {}", count);

}

fn read_from_file(file_name: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let nums: Vec<i32> = line.unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        data.push(nums);
    }

    return Ok(data)
}   

fn check_record(record: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = record.windows(2)
            .map(|window| window[0] - window[1])
            .collect();

        if diffs.iter().all(|&x| x.abs().between(1, 3)) && 
           (diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0)) {
            return true;
        }
        return false;
}

fn classify_safety_records(data: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for record in data {
        if check_record(&record) {
            count += 1;
        }
    }

    return count;
}

fn classify_with_problem_dampener(data: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for record in data {
        if check_record(&record) {
             count += 1;
        }
        else {
            for i in 0..record.len() {
                let mut new_record = record.clone();
                new_record.remove(i);
                if check_record(&new_record) {
                    count += 1;
                    break;
                }
            }
        }
    }

    return count;
}
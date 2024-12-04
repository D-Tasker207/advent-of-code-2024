use regex::Regex;
use std::fs;
use std::io::{self, Write};

fn main() {
    print!("Enter file name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");
    file_name = file_name.trim().to_string();

    let data = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let total = parse_multiplication(&data);
    println!("Total: {}", total);

    let total = parse_mult_with_conditions(&data);
    println!("Total with conditionals: {}", total);
}

fn parse_multiplication(data: &str) -> i32 {
    let mut total: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    for cap in re.captures_iter(data) {
        let (a, b): (i32, i32) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        total += a * b;
    }
    return total;
}

fn parse_mult_with_conditions(data: &str) -> i32 {
    let mut enable: bool = true;
    let mut total: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");
    for cap in re.captures_iter(data) {
        match &cap[0] {
            "do()" => enable = true,
            "don't()" => enable = false,
            _ => {
                if cap.get(1).is_some() && cap.get(2).is_some() {
                    let (a, b): (i32, i32) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
                    total += if enable { a * b } else { 0 };
                }
            }
        }
    }
    return total;
}

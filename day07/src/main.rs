use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};
// use std::collections::HashSet;

type ExpressionComponents = (u64, Vec<u64>);
static OPS: [fn(u64, u64) -> u64; 3] = [
    |a, b| a + b,
    |a, b| a * b,
    |a, b| concat(a, b),
];

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

    let expr_list: Vec<ExpressionComponents> = read_from_file(&file_name)
        .expect("Error reading from file");

    let sum = test_all_exprs(expr_list);
    println!("Sum of all expressions that evaluate to the target: {}", sum);

}

fn read_from_file(file_name: &str) -> io::Result<Vec<ExpressionComponents>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut expr_list: Vec<ExpressionComponents> = Vec::new();
    for line in lines {
        let line = line?;
        let components: Vec<&str> = line.split(":").collect();
        let target: u64 = components[0].parse::<u64>().unwrap();
        let expr: Vec<u64> = components[1]
            .trim()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        expr_list.push((target, expr));
    }

    Ok(expr_list)
}

fn test_all_exprs(expr_list: Vec<ExpressionComponents>) -> u64 {
    let mut sum = 0;
    for expr in expr_list.iter() {
        if eval_expr(expr.0, expr.1[0], expr.1[1..].to_vec()) {
            sum += expr.0;
        }
    }
    return sum;
}

fn eval_expr(target: u64, current: u64, nums: Vec<u64>) -> bool {
    if nums.is_empty() {
        return target == current;
    }

    OPS.iter().any(|op| {
        eval_expr(target, op(current, nums[0]), nums[1..].to_vec())
    })
}

    

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}
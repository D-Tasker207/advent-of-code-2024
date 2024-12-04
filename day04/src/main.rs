use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};

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

    let data = read_from_file(&file_name).expect("Error reading from file");

    let count = search_for_word(&data, "XMAS");
    println!("XMAS Count: {}", count);

    let count = search_for_x_shape(&data, "MAS");
    println!("X-MAS Count: {}", count);
}

fn read_from_file(file_name: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let chars: Vec<char> = line.unwrap().chars().collect();
        data.push(chars);
    }

    return Ok(data);
}

fn search_for_x_shape(data: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut count = 0;
    let mid_char = word.chars().nth(1).expect("Error getting middle char");

    for r in 0..data.len() {
        for c in 0..data[0].len() {
            if data[r][c] != mid_char {
                continue;
            }
            if validate_x_shape(data, word, (r, c)) {
                count += 1;
            }
        }
    }
    return count;
}

fn validate_x_shape(data: &Vec<Vec<char>>, word: &str, center_pos: (usize, usize)) -> bool {
    if !check_1_padding((data.len(), data[0].len()), center_pos) {
        return false;
    }
    let first_char = word.chars().nth(0).unwrap();
    let last_char = word.chars().nth(2).unwrap();
    for i in -1..2 {
        if i == 0 {
            continue;
        }
        let (a, b) = step_in_dir((center_pos.0, center_pos.1), (i, -1));
        let (c, d) = step_in_dir((center_pos.0, center_pos.1), (-i, 1));
        if !((data[a][b] == first_char && data[c][d] == last_char)
            || (data[a][b] == last_char && data[c][d] == first_char))
        {
            return false;
        }
    }
    return true;
}

fn search_for_word(data: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut count = 0;
    for r in 0..data.len() {
        for c in 0..data[0].len() {
            if data[r][c] != word.chars().nth(0).unwrap() {
                continue;
            }
            count += search_from_center(data, word, (r, c));
        }
    }
    return count;
}

fn search_from_center(data: &Vec<Vec<char>>, word: &str, c_pos: (usize, usize)) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            if (i == 0 && j == 0)
                || !check_step_inbounds((data.len(), data[0].len()), (c_pos.0, c_pos.1), (i, j))
            {
                continue;
            }
            let (a, b) = step_in_dir((c_pos.0, c_pos.1), (i, j));
            if data[a][b] != word.chars().nth(1).unwrap() {
                continue;
            }
            let start_idx = word
                .char_indices()
                .nth(2)
                .map(|(i, _)| i)
                .unwrap_or(word.len());
            if search_direction(data, word[start_idx..].chars().collect(), (a, b), (i, j)) {
                count += 1;
            }
        }
    }
    return count;
}
fn search_direction(
    data: &Vec<Vec<char>>,
    word_chars: Vec<char>,
    start_pos: (usize, usize),
    dir: (i32, i32),
) -> bool {
    let mut pos = start_pos;
    let mut i = 0;

    if word_chars.len() == 0 {
        return true;
    }

    while i < word_chars.len() {
        if check_step_inbounds((data.len(), data[0].len()), pos, dir) {
            pos = step_in_dir(pos, dir);
        } else {
            break;
        }
        if data[pos.0][pos.1] == word_chars[i] {
            i += 1;
            if i == word_chars.len() {
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}

fn check_1_padding(data_shape: (usize, usize), pos: (usize, usize)) -> bool {
    return pos.0 >= 1 && pos.0 < data_shape.0 - 1 && pos.1 >= 1 && pos.1 < data_shape.1 - 1;
}

fn check_step_inbounds(data_shape: (usize, usize), pos: (usize, usize), dir: (i32, i32)) -> bool {
    return !((pos.0 as i32 + dir.0 < 0 || pos.0 as i32 + dir.0 >= data_shape.0 as i32)
        || (pos.1 as i32 + dir.1 < 0 || pos.1 as i32 + dir.1 >= data_shape.1 as i32));
}

fn step_in_dir(pos: (usize, usize), dir: (i32, i32)) -> (usize, usize) {
    return (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    );
}

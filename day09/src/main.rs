use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Clone)]
struct DataBlock {
    id: i64,
    start_idx: u64,
    size: u64,
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

    let data = read_from_file(&file_name)
        .expect("Error reading from file");
    let blocks = parse_data_to_blocks(data);
    let fragmented_blocks = consolidate_stream(&blocks);
    let unfragmented_blocks = move_whole_blocks(&blocks);

    let sum = sum_blocks(&fragmented_blocks);
    println!("Fragmented Sum: {}", sum);

    let sum = sum_blocks(&unfragmented_blocks);
    println!("Unfragmented Sum: {}", sum);
}

fn read_from_file(file_name: &str) -> io::Result<Vec<u64>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();

    let mut num_list = Vec::new();
    for line in lines {
        num_list = line.unwrap()
            .chars()
            .map(|v| v.to_digit(10).unwrap() as u64)
            .collect();
    }

    Ok(num_list)
}

fn parse_data_to_blocks(data: Vec<u64>) -> Vec<DataBlock> {
    let mut cur_id = 0;
    data.into_iter()
        .enumerate()
        .filter(|(_, v)| *v > 0)
        .scan(0, |idx, (i, value)| {
            let block = if i % 2 == 0 {
                let new_block = DataBlock {
                    id: cur_id,
                    start_idx: *idx,
                    size: value,
                };
                cur_id += 1;
                new_block
            } else {
                DataBlock {
                    id: -1,
                    start_idx: *idx,
                    size: value,
                }
            };
            *idx += value;
            Some(block)
        })
        .collect()

}

fn consolidate_stream(blocks: &Vec<DataBlock>) -> Vec<DataBlock> {
    let mut blocks = blocks.clone();
    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].id >= 0 {
            i += 1;
            continue;
        } else {
            let new_blocks = fill_empty_block(&mut blocks, i);
            blocks.splice(i..i+1, new_blocks);
        }
        blocks = prune_and_merge_blocks(&blocks);
        i += 1;
    }
    blocks
}

fn fill_empty_block(blocks: &mut Vec<DataBlock>, cur_idx: usize) -> Vec<DataBlock> {
    let mut new_blocks = Vec::new();
    let last_data_block_idx = get_last_data_block(&blocks);
    if cur_idx >= last_data_block_idx {
        return vec![];
    }
    let(remaining_blocks, filler_block) = blocks.split_at_mut(last_data_block_idx);
    let current_block = &mut remaining_blocks[cur_idx];
    let filler_block = &mut filler_block[0];
    
    if current_block.size <= filler_block.size {
        new_blocks.push(DataBlock {
            id: filler_block.id,
            start_idx: current_block.start_idx,
            size: current_block.size,
        });
        filler_block.size -= current_block.size;
    } else {
        new_blocks.push(DataBlock {
            id: filler_block.id,
            start_idx: current_block.start_idx,
            size: filler_block.size,
        });
        let remaining_size = current_block.size - filler_block.size;
        filler_block.size = 0;
        new_blocks.push(DataBlock {
            id: -1,
            start_idx: current_block.start_idx + filler_block.size,
            size: remaining_size,
        });
    }

    new_blocks
}

fn prune_and_merge_blocks(blocks: &Vec<DataBlock>) -> Vec<DataBlock> {
    let blocks = blocks.iter().filter(|block| block.size > 0).collect::<Vec<_>>();

    let mut merged_blocks = Vec::new();
    let mut i = 0;
    while i < blocks.len() {
        let mut current = blocks[i].clone();
        while i + 1 < blocks.len() && current.id == blocks[i + 1].id {
            current.size += blocks[i + 1].size;
            i += 1;
        }
        merged_blocks.push(current);
        i += 1;
    }

    merged_blocks
}

fn get_last_data_block(blocks: &Vec<DataBlock>) -> usize {
    blocks
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, block)| block.id > 0 && block.size > 0)
        .map(|(index, _)| index)
        .next()
        .unwrap()
}

fn sum_blocks(blocks: &Vec<DataBlock>) -> u64 {
    let mut idx = 0;
    let mut sum = 0;
    for block in blocks {
        if block.id < 0 {
            idx += block.size;
            continue;
        }
        for _ in 0..block.size {
            sum += idx * block.id as u64;
            idx += 1;
        }
    }
    sum
}

fn move_whole_blocks(blocks: &Vec<DataBlock>) -> Vec<DataBlock> {
    let mut blocks = blocks.clone();
    let mut i = blocks.len();
    while i > 0 {
        i -= 1;
        let block = &blocks[i];
        if block.id < 0 {
            continue;
        }

        let left_free_space = blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.id < 0 && b.size >= block.size && b.start_idx < block.start_idx)
            .collect::<Vec<_>>();

        if left_free_space.len() > 0 {
            let (idx, free_block) = left_free_space[0];
            let new_blocks = vec![
                DataBlock {
                    id: block.id,
                    start_idx: free_block.start_idx,
                    size: block.size,
                },
                DataBlock {
                    id: -1,
                    start_idx: free_block.start_idx + block.size,
                    size: free_block.size - block.size,
                }
            ];
            blocks[i].id = -1;
            blocks.splice(idx..idx + 1, new_blocks);
            i += 1;
        }
    }

    prune_and_merge_blocks(&blocks)
}
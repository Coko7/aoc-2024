use std;

use itertools::Itertools;

advent_of_code::solution!(9);

fn init_file_system(disk_map: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut file_id_serial = 0;

    for (idx, block_length) in disk_map.chars().enumerate() {
        let is_free = idx % 2 == 1;
        let block_length: u32 = block_length.to_digit(10).unwrap();

        for _i in 0..block_length {
            let block: String = match is_free {
                true => ".".to_string(),
                false => file_id_serial.to_string(),
            };

            blocks.push(block);
        }

        if !is_free {
            file_id_serial += 1;
        }
    }

    blocks
}

fn optimize_file_system(mut file_system: Vec<String>) -> Vec<String> {
    loop {
        let lfb = find_last_file_block(&file_system);
        let ffb = find_first_free_block(&file_system);

        if lfb.is_none() || ffb.is_none() {
            break;
        }

        let file_block = lfb.unwrap();
        let free_block = ffb.unwrap();

        if free_block >= file_block.1 {
            break;
        }

        let _ = std::mem::replace(&mut file_system[free_block], file_block.0);
        let _ = std::mem::replace(&mut file_system[file_block.1], String::from("."));
    }

    file_system
}

fn find_last_file_block(blocks: &Vec<String>) -> Option<(String, usize)> {
    let len = blocks.len();

    for (inv_pos, block) in blocks.iter().rev().enumerate() {
        if block != "." {
            let pos = len - 1 - inv_pos;
            return Some((block.to_string(), pos));
        }
    }

    None
}

fn find_first_free_block(blocks: &Vec<String>) -> Option<usize> {
    blocks.iter().position(|b| b == ".")
}

fn compute_checksum(blocks: &Vec<String>) -> u64 {
    let mut checksum = 0;

    for (idx, block_id) in blocks.iter().enumerate() {
        let idx: u64 = idx.try_into().unwrap();
        match block_id.as_str() {
            "." => continue,
            val => {
                let file_id: u64 = val.parse().unwrap();
                let check = idx * file_id;
                checksum += check;
            }
        };
    }

    checksum
}

fn optimize_file_system2(mut file_system: String) -> String {
    let mut skipped_files: Vec<String> = Vec::new();

    let mut i = 0;
    loop {
        i += 1;
        println!("{}", file_system);

        if i > 50 {
            break;
        }

        let last_file: Option<String> = file_system
            .split(".")
            .filter(|s| !s.is_empty())
            .filter(|s| !skipped_files.contains(&s.to_string()))
            .last()
            .map(|s| s.to_string());

        if last_file.is_none() {
            break;
        }

        let last_file = &last_file.unwrap()[1..];

        println!("last: {}", last_file);

        let free_space = last_file.split(",").map(|_| ".").join(",");
        if !file_system.contains(&free_space) {
            skipped_files.push(last_file.to_string());
            continue;
        }

        file_system =
            file_system
                .replacen(&last_file, &free_space, 1)
                .replacen(&free_space, &last_file, 1);
    }

    file_system
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = init_file_system(input.trim());
    let blocks = optimize_file_system(blocks);
    let checksum = compute_checksum(&blocks);

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = init_file_system(input.trim());
    let fs = blocks.join(",");
    let blocks = optimize_file_system2(fs);
    // let checksum = compute_checksum(&blocks);

    // Some(checksum)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

// fn init_fs_desc(input: &str) -> Vec<(i32, usize)> {
//     let mut sectors = Vec::new();
//
//     let mut file_id_serial = 0;
//     for (i, c) in input.chars().enumerate() {
//         let is_free = i % 2 == 1;
//         let sector_len: usize = c.to_digit(10).unwrap().try_into().unwrap();
//
//         let sector_id: i32 = if is_free { -1 } else { file_id_serial };
//
//         let sector = (sector_id, sector_len);
//         sectors.push(sector);
//
//         if !is_free {
//             file_id_serial += 1;
//         }
//     }
//
//     sectors
// }
//
//
//

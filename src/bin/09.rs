use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Debug)]
struct FileSystem {
    id_serial: u32,
    sectors: Vec<Sector>,
}

impl FileSystem {
    fn from_input(input: &str) -> FileSystem {
        let mut sectors: Vec<Sector> = Vec::new();
        let mut id_serial = 0;
        let mut file_id_serial = 0;

        let mut true_pos = 0;
        for (idx, sector_length) in input.chars().enumerate() {
            let is_free = idx % 2 == 1;
            let sector_length: usize = sector_length.to_digit(10).unwrap() as usize;
            let id = if is_free { None } else { Some(file_id_serial) };

            sectors.push(Sector {
                id: id_serial,
                file_id: id,
                start: true_pos,
                length: sector_length,
            });

            id_serial += 1;
            true_pos += sector_length;

            if !is_free {
                file_id_serial += 1;
            }
        }

        FileSystem { id_serial, sectors }
    }

    fn add_free(&mut self, start: usize, length: usize) {
        let sector = Sector {
            id: self.id_serial,
            file_id: None,
            start,
            length,
        };
        self.sectors.push(sector);
        self.id_serial += 1;
    }

    fn merge_free(&mut self) {
        let mut i = 0;
        while let Some(first_free_id) = self.find_nth_free(i) {
            // println!("{}", i);
            if let Some(next_free_id) = self.find_nth_free(i + 1) {
                let next_free = self.find_sector(next_free_id).unwrap();
                let next_free_start = next_free.start;
                let next_free_length = next_free.length;

                let current_free = self.find_sector_mut(first_free_id).unwrap();
                if current_free.end() == next_free_start {
                    current_free.length += next_free_length;
                    let next_free_pos = self
                        .sectors
                        .iter()
                        .position(|s| s.id == next_free_id)
                        .unwrap();

                    self.sectors.remove(next_free_pos);
                } else {
                    i += 1;
                }
            } else {
                break;
            }
        }
    }

    fn optimize(&mut self) {
        let mut i = 0;

        while let Some(last_file_id) = self.find_nth_last_file(i) {
            let last_file = self.find_sector(last_file_id).unwrap();
            let last_file_start = last_file.start;
            let last_file_len = last_file.length;

            if let Some(free_sector_id) = self.find_free_sector(last_file_start, last_file_len) {
                let free_sector = self.find_sector_mut(free_sector_id).unwrap();
                let free_sector_old_start = free_sector.start;
                free_sector.start += last_file_len;
                free_sector.length -= last_file_len;

                let last_file = self.find_sector_mut(last_file_id).unwrap();
                last_file.start = free_sector_old_start;

                self.add_free(last_file_start, last_file_len);
                // self.merge_free();
                // self.display();
            } else {
                i += 1;
            }
        }
    }

    fn find_sector(&self, id: u32) -> Option<&Sector> {
        self.sectors.iter().find(|s| s.id == id)
    }

    fn find_sector_mut(&mut self, id: u32) -> Option<&mut Sector> {
        self.sectors.iter_mut().find(|s| s.id == id)
    }

    fn find_nth_free(&self, nth: usize) -> Option<u32> {
        self.sectors
            .iter()
            .filter(|s| s.is_free())
            .sorted_by_key(|&s| s.start)
            .map(|s| s.id)
            .nth(nth)
    }

    fn find_nth_last_file(&self, nth: usize) -> Option<u32> {
        self.sectors
            .iter()
            .filter(|s| !s.is_free())
            .sorted_by_key(|&s| s.start)
            .map(|s| s.id)
            .rev()
            .nth(nth)
    }

    fn find_free_sector(&self, file_start: usize, file_size: usize) -> Option<u32> {
        self.sectors
            .iter()
            .filter(|s| s.is_free())
            .sorted_by_key(|&s| s.start)
            .find(|s| s.start < file_start && s.length >= file_size)
            .map(|s| s.id)
    }

    fn compute_checksum(&self) -> u64 {
        let sorted_sectors = self.sectors.iter().sorted_by_key(|s| s.start);

        let mut res = 0;
        for sector in sorted_sectors {
            if sector.is_free() {
                continue;
            }

            for i in 0..sector.length {
                let pos = (i + sector.start) as u64;
                let file_id = sector.file_id.unwrap() as u64;
                res += pos * file_id;
            }
        }

        res
    }

    fn display(&self) {
        let _ = self
            .sectors
            .iter()
            .sorted_by_key(|s| s.start)
            .for_each(|s| s.display());
        println!("");
    }
}

#[derive(Clone, Debug)]
struct Sector {
    id: u32,
    file_id: Option<u32>,
    start: usize,
    length: usize,
}

fn num_to_b64_char(num: usize) -> char {
    let b64_chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz@#";
    b64_chars.chars().nth(num).unwrap_or('?')
}

impl Sector {
    fn is_free(&self) -> bool {
        self.file_id.is_none()
    }

    fn end(&self) -> usize {
        self.start + self.length
    }

    fn display(&self) {
        let id = if let Some(id) = self.file_id {
            num_to_b64_char(id as usize)
        } else {
            '.'
        }
        .to_string();

        let str = id.repeat(self.length);
        print!("{}", str);
    }
}

fn init_fs_blocks(disk_map: &str) -> Vec<String> {
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

fn optimize_fs_blocks(mut file_system: Vec<String>) -> Vec<String> {
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

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = init_fs_blocks(input.trim());
    let blocks = optimize_fs_blocks(blocks);
    let checksum = compute_checksum(&blocks);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fs = FileSystem::from_input(input.trim());
    fs.optimize();
    Some(fs.compute_checksum())
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

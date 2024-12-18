use core::panic;

use advent_of_code::{pos2idx, Map2D, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Box,
    Wall,
    Floor,
}

#[derive(Debug)]
struct Map {
    base: Map2D<TileType>,
    robot_pos: Pos2D,
}

impl Map {
    fn get_boxes(&self) -> Vec<Pos2D> {
        self.base
            .tiles
            .iter()
            .positions(|&tile| tile == TileType::Box)
            .map(|pos| self.base.idx2pos(pos))
            .collect()
    }

    fn get_gps_coord(&self, position: &Pos2D) -> u32 {
        (position.y * 100 + position.x) as u32
    }

    fn move_box(&mut self, position: &Pos2D, offset: &Pos2D) -> bool {
        let next_pos = position.add(offset);
        let next_tile = self.base.get(&next_pos).unwrap();

        let cur_idx = self.base.pos2idx(position).unwrap();
        let next_idx = self.base.pos2idx(&next_pos).unwrap();

        match next_tile {
            TileType::Box => {
                if self.move_box(&next_pos, offset) {
                    self.base.tiles[cur_idx] = TileType::Floor;
                    self.base.tiles[next_idx] = TileType::Box;
                    true
                } else {
                    false
                }
            }
            TileType::Wall => false,
            TileType::Floor => {
                self.base.tiles[cur_idx] = TileType::Floor;
                self.base.tiles[next_idx] = TileType::Box;
                true
            }
        }
    }

    fn move_robot(&mut self, offset: &Pos2D) {
        let next_pos = self.robot_pos.add(offset);
        if !self.base.is_valid_pos(&next_pos) {
            return;
        }

        let next_tile = self.base.get(&next_pos).unwrap();
        match next_tile {
            TileType::Box => {
                if self.move_box(&next_pos, offset) {
                    self.robot_pos = next_pos;
                }
            }
            TileType::Wall => {}
            TileType::Floor => self.robot_pos = next_pos,
        }
    }

    fn display(&self) {
        for y in 0..self.base.height {
            for x in 0..self.base.width {
                let pos = Pos2D::new(x as i32, y as i32);
                let tile = self.base.get(&pos).unwrap();
                if self.robot_pos.x == pos.x && self.robot_pos.y == pos.y {
                    print!("@");
                } else {
                    let c = match tile {
                        TileType::Box => 'O',
                        TileType::Wall => '#',
                        TileType::Floor => '.',
                    };
                    print!("{}", c);
                }
            }
            println!("");
        }
        println!("");
    }

    fn from_input(input: &str) -> Map {
        let height = input.lines().count();
        let width = input.lines().last().unwrap().chars().count();

        let mut tiles = vec![TileType::Floor; height * width];
        let mut robot_pos = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Pos2D::new(x as i32, y as i32);
                if c == '@' {
                    robot_pos = Some(position);
                } else {
                    let tile = match c {
                        '#' => TileType::Wall,
                        'O' => TileType::Box,
                        '.' => TileType::Floor,
                        _ => panic!("Unknown tile: {}", c),
                    };

                    let idx = pos2idx(&position, width, height).unwrap();
                    tiles[idx] = tile;
                }
            }
        }

        Map {
            base: Map2D {
                width,
                height,
                tiles,
            },
            robot_pos: robot_pos.unwrap(),
        }
    }

    fn process_moves(&mut self, moves: Vec<char>) {
        // println!("Initial state");
        // self.display();
        for char_move in moves {
            let offset_pos = move2pos(char_move);
            // println!("Move {}:", char_move);
            self.move_robot(&offset_pos);
            // self.display();
        }
    }
}

fn move2pos(c: char) -> Pos2D {
    match c {
        '^' => Pos2D::new(0, -1),
        '>' => Pos2D::new(1, 0),
        'v' => Pos2D::new(0, 1),
        '<' => Pos2D::new(-1, 0),
        _ => panic!("Unknown move char: {}", c),
    }
}

fn parse_input(input: &str) -> (Map, Vec<char>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let map_input = parts.first().unwrap();
    let moves: Vec<char> = parts
        .last()
        .unwrap()
        .replace('\n', "")
        .trim()
        .chars()
        .collect();

    let map = Map::from_input(map_input);
    (map, moves)
}

fn transform_map_input(input: &str) -> String {
    input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, moves) = parse_input(input);
    map.process_moves(moves);
    Some(
        map.get_boxes()
            .iter()
            .map(|box_pos| map.get_gps_coord(box_pos))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

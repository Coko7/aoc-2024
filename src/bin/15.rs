use core::panic;

use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add_pos(&self, position: &Pos) -> Pos {
        Pos {
            x: self.x + position.x,
            y: self.y + position.y,
        }
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Box,
    Wall,
    Floor,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
    robot_pos: Pos,
}

impl Map {
    fn pos_idx(&self, position: &Pos) -> usize {
        pos_idx(position, self.width, self.height)
    }

    fn idx_xy(&self, idx: usize) -> Pos {
        let x = (idx % self.width) as i32;
        let y = (idx / self.height) as i32;
        Pos::new(x, y)
    }

    fn is_valid_pos(&self, position: &Pos) -> bool {
        position.x >= 0
            && position.x < (self.width as i32)
            && position.y >= 0
            && position.y < (self.height as i32)
    }

    fn get_boxes(&self) -> Vec<Pos> {
        self.tiles
            .iter()
            .positions(|&tile| tile == TileType::Box)
            .map(|pos| self.idx_xy(pos))
            .collect()
    }

    fn get_gps_coord(&self, position: &Pos) -> u32 {
        (position.y * 100 + position.x) as u32
    }

    fn get_tile(&self, position: &Pos) -> TileType {
        self.tiles[self.pos_idx(position)]
    }

    fn move_box(&mut self, position: &Pos, offset: &Pos) -> bool {
        let next_pos = position.add_pos(offset);
        let next_tile = self.get_tile(&next_pos);

        match next_tile {
            TileType::Box => {
                if self.move_box(&next_pos, offset) {
                    self.tiles[pos_idx(position, self.width, self.height)] = TileType::Floor;
                    self.tiles[pos_idx(&next_pos, self.width, self.height)] = TileType::Box;
                    true
                } else {
                    false
                }
            }
            TileType::Wall => false,
            TileType::Floor => {
                self.tiles[pos_idx(position, self.width, self.height)] = TileType::Floor;
                self.tiles[pos_idx(&next_pos, self.width, self.height)] = TileType::Box;
                true
            }
        }
    }

    fn move_robot(&mut self, offset: &Pos) {
        let next_pos = self.robot_pos.add_pos(offset);
        if !self.is_valid_pos(&next_pos) {
            return;
        }

        let next_tile = self.get_tile(&next_pos);
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
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x as i32, y as i32);
                let tile = self.get_tile(&pos);
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
}

impl Map {
    fn from_input(input: &str) -> Map {
        let height = input.lines().count();
        let width = input.lines().last().unwrap().chars().count();

        let mut tiles = vec![TileType::Floor; height * width];
        let mut robot_pos = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Pos::new(x as i32, y as i32);
                if c == '@' {
                    robot_pos = Some(position);
                } else {
                    let tile = match c {
                        '#' => TileType::Wall,
                        'O' => TileType::Box,
                        '.' => TileType::Floor,
                        _ => panic!("Unknown tile: {}", c),
                    };

                    tiles[pos_idx(&position, width, height)] = tile;
                }
            }
        }

        Map {
            width,
            height,
            tiles,
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

fn pos_idx(position: &Pos, width: usize, height: usize) -> usize {
    assert!(
        position.x >= 0 && position.x < width as i32,
        "x must be within 0..{}",
        width
    );
    assert!(
        position.y >= 0 && position.y < height as i32,
        "y must be within 0..{}",
        height
    );

    (position.y as usize * height) + position.x as usize
}

fn move2pos(c: char) -> Pos {
    match c {
        '^' => Pos::new(0, -1),
        '>' => Pos::new(1, 0),
        'v' => Pos::new(0, 1),
        '<' => Pos::new(-1, 0),
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

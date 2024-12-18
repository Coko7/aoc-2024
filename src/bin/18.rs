use core::panic;
use std::usize;

use advent_of_code::{idx2pos, pos2idx, Pos2D};

advent_of_code::solution!(18);

type Node = usize;
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<char>,
}

impl Map {
    fn pos2idx(&self, position: &Pos2D) -> Node {
        pos2idx(position, self.width, self.height).unwrap()
    }

    fn idx2pos(&self, idx: usize) -> Pos2D {
        idx2pos(idx, self.width, self.height)
    }

    fn start_idx(&self) -> usize {
        0
    }

    fn goal_idx(&self) -> usize {
        self.tiles.len() - 1
    }

    fn dist(&self, a: Node, b: Node) -> i32 {
        let a_pos = self.idx2pos(a);
        let b_pos = self.idx2pos(b);
        (a_pos.dist(&b_pos) * 10.) as i32
    }

    fn calc_h(&self, node: Node) -> i32 {
        assert!(node < self.tiles.len(), "Invalid node: {}", node);

        self.dist(node, self.start_idx())
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        self.idx2pos(node)
            .neighbors(false)
            .iter()
            .filter(|pos| self.is_valid_pos(&pos))
            .map(|pos| self.pos2idx(pos))
            .filter(|&node| self.tiles[node] != '#')
            .collect()
    }

    pub fn is_valid_pos(&self, position: &Pos2D) -> bool {
        position.x >= 0
            && position.x < (self.width as i32)
            && position.y >= 0
            && position.y < (self.height as i32)
    }

    fn display(&self) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 {
                println!("");
            }
            print!("{}", tile);
        }
        println!("\n");
    }

    fn display_path(&self, path: &Vec<Node>) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 {
                println!("");
            }

            if path.contains(&idx) {
                print!("O");
            } else {
                print!("{}", tile);
            }
        }
        println!("\n");
    }
}

fn preprocess_input(input: &str) -> ((usize, usize, usize), String) {
    let meta: Vec<&str> = input.lines().next().unwrap().split(",").collect();

    let width = meta.get(0).unwrap().parse::<usize>().unwrap();
    let height = meta.get(1).unwrap().parse::<usize>().unwrap();
    let simulated_bytes = meta.get(2).unwrap().parse::<usize>().unwrap();

    let remain = input.lines().skip(2).collect::<Vec<&str>>().join("\n");
    ((width, height, simulated_bytes), remain)
}

fn create_map(input: &str, width: usize, height: usize, simulated_bytes: usize) -> Map {
    let mut tiles = vec!['.'; width * height];
    let obstacles: Vec<Pos2D> = input
        .lines()
        .take(simulated_bytes)
        .map(|l| parse_coords(l))
        .collect();

    for obstacle in obstacles.iter() {
        let pos = obstacle;
        let idx = pos2idx(&pos, width, height).unwrap();
        tiles[idx] = '#';
    }

    Map {
        width,
        height,
        tiles,
    }
}

fn find_min_node(nodes: &Vec<Node>, scores: &Vec<i32>) -> Option<Node> {
    let mut min_score = None;
    let mut mine_node = None;
    for &node in nodes.iter() {
        let score = scores[node];
        if min_score.is_none() || score < min_score.unwrap() {
            min_score = Some(score);
            mine_node = Some(node);
        }
    }

    mine_node
}

fn reconstruct_path(came_from: &Vec<Option<Node>>, current: Node) -> Vec<Node> {
    let mut current_node = current;
    let mut total_path = vec![current];
    loop {
        if let Some(current) = came_from[current_node] {
            total_path.insert(0, current);
            current_node = current;
        } else {
            break;
        }
    }

    total_path
}

fn a_star(map: &Map) -> Result<Vec<Node>, ()> {
    let mut open_set: Vec<Node> = vec![map.start_idx()];
    let mut came_from: Vec<Option<Node>> = vec![None; map.tiles.len()];

    let mut g_scores = vec![i32::MAX; map.tiles.len()];
    g_scores[map.start_idx()] = 0;

    let mut f_scores = vec![i32::MAX; map.tiles.len()];
    f_scores[map.start_idx()] = map.calc_h(map.start_idx());

    while !open_set.is_empty() {
        let current = find_min_node(&open_set, &f_scores).unwrap();

        if current == map.goal_idx() {
            return Ok(reconstruct_path(&came_from, current));
        }

        // Remove current from openSet
        open_set = open_set
            .iter()
            .filter(|&n| *n != current)
            .cloned()
            .collect();

        let neighbors = map.get_neighbors(current);

        for &neighbor in neighbors.iter() {
            let tentative_g_score = g_scores[current] + map.dist(current, neighbor);
            if tentative_g_score < g_scores[neighbor] {
                came_from[neighbor] = Some(current);
                g_scores[neighbor] = tentative_g_score;
                f_scores[neighbor] = tentative_g_score + map.calc_h(neighbor);
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    Err(())
}

fn parse_coords(input: &str) -> Pos2D {
    let (x, y) = input
        .split_once(',')
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unwrap();

    Pos2D::new(x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (meta, input) = preprocess_input(input);
    let width = meta.0;
    let height = meta.1;
    let simulated_bytes = meta.2;

    let map = create_map(&input, width, height, simulated_bytes);

    if let Ok(shortest_path) = a_star(&map) {
        // start node does not count as a step
        let steps = shortest_path.len() - 1;
        return Some(steps as u32);
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    let (meta, input) = preprocess_input(input);
    let width = meta.0;
    let height = meta.1;

    let max_bytes = input.lines().count();
    for i in 0..=max_bytes {
        let map = create_map(&input, width, height, i);
        match a_star(&map) {
            Ok(_) => {}
            _err => match input.lines().nth(i - 1).map(|l| parse_coords(l)) {
                Some(pos) => return Some(format!("{},{}", pos.x, pos.y)),
                None => panic!("parse_coords should always work here!"),
            },
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}

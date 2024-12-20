use advent_of_code::{idx2pos, pos2idx, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(20);

type Node = usize;
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<char>,
    start: Node,
    end: Node,
}

impl Map {
    fn pos2idx(&self, position: &Pos2D) -> Node {
        pos2idx(position, self.width, self.height).unwrap()
    }

    fn idx2pos(&self, idx: usize) -> Pos2D {
        idx2pos(idx, self.width, self.height)
    }

    fn start_idx(&self) -> usize {
        self.start
    }

    fn goal_idx(&self) -> usize {
        self.end
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

    fn get_walls_with_n_liberties(&self, min: usize, max: usize) -> Vec<Node> {
        assert!(
            min >= 0,
            "A wall cannot have a negative amount of liberties!"
        );
        assert!(
            max <= 4,
            "A wall cannot have more than 4 liberties as diagonals are not counted!"
        );

        let all_walls = self.tiles.iter().positions(|&n| n == '#');
        let mut filtered_walls = vec![];

        for wall_idx in all_walls {
            let neighbors: Vec<Node> = self.get_neighbors(wall_idx);
            let liberties = neighbors.iter().filter(|&n| self.tiles[*n] != '#').count();
            if liberties >= min && liberties <= max {
                filtered_walls.push(wall_idx);
            }
        }

        filtered_walls
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

        open_set.retain(|&n| n != current);

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

fn create_map(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().last().unwrap().chars().count();
    let tiles: Vec<char> = input.replace('\n', "").chars().collect();

    let start = tiles.iter().position(|&c| c == 'S').unwrap();
    let end = tiles.iter().position(|&c| c == 'E').unwrap();

    Map {
        width,
        height,
        tiles,
        start,
        end,
    }
}

fn gained_ps(legit_path: &Vec<Node>, cheat_path: &Vec<Node>) -> i32 {
    (legit_path.len() as i32) - (cheat_path.len() as i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = create_map(input);
    let legit_path = a_star(&map).unwrap();

    // Liberties:
    // - 0: Wall is enclosed by four walls
    // - 1: Wall is enclosed by three walls
    // - 2: Wall is enclosed by two walls
    // - 3: Wall is enclosed by only one wall (corner)
    // - 4: Wall is a pillar (no surrounding wall)
    // Here is what happens if you try to remove walls with N liberties:
    // - 0: No new path created (creates empty spot enclosed by walls)
    // - 1: No new path created (creates empty spot that ends in a dead end)
    // - 2: Removing wall may allow to jump to the other side, possibly saving a LOT of time
    // - 3: Wall is a corner and removing it will slighlty improve the path
    // - 4: Wall was a pillar (non blocking) so removing it does not improve anything
    // The walls with exactly 2 liberties are the best ones to remove.
    let cheatable_walls = map.get_walls_with_n_liberties(2, 3);
    // println!("Cheatable walls: {}", cheatable_walls.len());

    let mut cheated_paths = vec![];
    for &wall_idx in cheatable_walls.iter() {
        map.tiles[wall_idx] = '.';
        let cheated_shortest_path = a_star(&map).unwrap();
        // println!(
        //     "After cheating at ({}), best path is `{}` picoseconds (-{} ps)",
        //     wall_idx,
        //     cheated_shortest_path.len() - 1,
        //     legit_path.len() - cheated_shortest_path.len()
        // );
        cheated_paths.push(cheated_shortest_path);
        map.tiles[wall_idx] = '#';
    }

    // For actual input, use gained_ps >= 100
    let cheated_paths: Vec<_> = cheated_paths
        .iter()
        .filter(|path| gained_ps(&legit_path, path) >= 1)
        .collect();

    Some(cheated_paths.len() as u32)
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
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

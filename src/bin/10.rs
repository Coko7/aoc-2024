advent_of_code::solution!(10);

struct Map {
    grid: Vec<Vec<u32>>,
    trail_heads: Vec<Pos>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_valid_pos(&self, pos: &Pos) -> bool {
        let width = self.width.try_into().unwrap();
        let height = self.height.try_into().unwrap();

        pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
    }

    fn get_height(&self, pos: &Pos) -> Option<u32> {
        if self.is_valid_pos(&pos) {
            let x: usize = pos.x.try_into().unwrap();
            let y: usize = pos.y.try_into().unwrap();
            return Some(self.grid[y][x]);
        }

        None
    }

    fn is_move_allowed(&self, start: &Pos, end: &Pos) -> bool {
        if !self.is_valid_pos(&start) || !self.is_valid_pos(&end) {
            return false;
        }

        let start_height = self.get_height(&start).unwrap();
        let end_height = self.get_height(&end).unwrap();

        // println!("start {}: end {}", start_height, end_height);

        end_height == start_height + 1
    }
}

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn right(&self, amount: i32) -> Pos {
        Pos {
            x: self.x + amount,
            y: self.y,
        }
    }

    fn left(&self, amount: i32) -> Pos {
        Pos {
            x: self.x - amount,
            y: self.y,
        }
    }

    fn top(&self, amount: i32) -> Pos {
        Pos {
            x: self.x,
            y: self.y - amount,
        }
    }

    fn bot(&self, amount: i32) -> Pos {
        Pos {
            x: self.x,
            y: self.y + amount,
        }
    }
}

fn init_map(input: &str) -> Map {
    let mut grid = Vec::new();
    let mut trail_heads = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, char) in line.chars().enumerate() {
            let height: u32 = char.to_digit(10).unwrap();
            row.push(height);

            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            if height == 0 {
                trail_heads.push(Pos { x, y });
            }
        }
        grid.push(row);
    }

    let height = input.lines().count();
    let width = input.lines().last().unwrap().len();

    Map {
        grid,
        trail_heads,
        height,
        width,
    }
}

fn get_next_pos_vec(current: &Pos, map: &Map) -> Vec<Pos> {
    let mut res = Vec::new();

    let right = current.right(1);
    let bot = current.bot(1);
    let left = current.left(1);
    let top = current.top(1);

    if map.is_move_allowed(&current, &right) {
        res.push(right);
    }
    if map.is_move_allowed(&current, &bot) {
        res.push(bot);
    }
    if map.is_move_allowed(&current, &left) {
        res.push(left);
    }
    if map.is_move_allowed(&current, &top) {
        res.push(top);
    }

    res
}

fn calculate_score(trail_head: &Pos, map: &Map) -> u32 {
    let mut to_visit: Vec<Pos> = Vec::new();
    let mut visited: Vec<Pos> = Vec::new();

    to_visit.push(trail_head.clone());

    let mut score = 0;

    loop {
        let current = to_visit.pop();
        if current.is_none() {
            break;
        }

        let current = current.unwrap();
        // println!("current: {:?}", current);

        if map.get_height(&current).unwrap() == 9 {
            score += 1;
        }

        visited.push(current.clone());

        let candidates = get_next_pos_vec(&current, &map);

        // println!("candidates: {:?}", candidates);
        let valid_cands = candidates
            .iter()
            .filter(|p| !visited.iter().any(|v| v.x == p.x && v.y == p.y));

        // println!("{:?}", valid_cands);

        for candidate in valid_cands {
            to_visit.push(candidate.clone());
        }
    }

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = init_map(input);

    let mut total_score = 0;
    for trail_head in map.trail_heads.iter() {
        let score = calculate_score(&trail_head, &map);
        // println!("trail {:?} has score: {}", trail_head, score);
        total_score += score;
    }

    Some(total_score)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

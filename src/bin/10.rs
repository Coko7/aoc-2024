use advent_of_code::Pos2D;

advent_of_code::solution!(10);

struct Map {
    grid: Vec<Vec<u32>>,
    trail_heads: Vec<Pos2D>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_valid_pos(&self, pos: &Pos2D) -> bool {
        let width = self.width.try_into().unwrap();
        let height = self.height.try_into().unwrap();

        pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
    }

    fn get_height(&self, pos: &Pos2D) -> Option<u32> {
        if self.is_valid_pos(&pos) {
            let x: usize = pos.x.try_into().unwrap();
            let y: usize = pos.y.try_into().unwrap();
            return Some(self.grid[y][x]);
        }

        None
    }

    fn is_move_allowed(&self, start: &Pos2D, end: &Pos2D) -> bool {
        if !self.is_valid_pos(&start) || !self.is_valid_pos(&end) {
            return false;
        }

        let start_height = self.get_height(&start).unwrap();
        let end_height = self.get_height(&end).unwrap();

        // println!("start {}: end {}", start_height, end_height);

        end_height == start_height + 1
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

            let x = x as i32;
            let y = y as i32;

            if height == 0 {
                trail_heads.push(Pos2D::new(x, y));
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

fn get_next_pos_vec(current: &Pos2D, map: &Map) -> Vec<Pos2D> {
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

fn calculate_score(trail_head: &Pos2D, map: &Map) -> u32 {
    let mut to_visit: Vec<Pos2D> = Vec::new();
    let mut visited: Vec<Pos2D> = Vec::new();

    to_visit.push(trail_head.clone());

    let mut score = 0;

    loop {
        let current = to_visit.pop();
        if current.is_none() {
            break;
        }

        let current = current.unwrap();

        if map.get_height(&current).unwrap() == 9 {
            score += 1;
        }

        visited.push(current.clone());

        let candidates = get_next_pos_vec(&current, &map);
        let valid_cands = candidates
            .iter()
            .filter(|p| !visited.iter().any(|v| v.x == p.x && v.y == p.y));

        for candidate in valid_cands {
            to_visit.push(candidate.clone());
        }
    }

    score
}

fn calculate_score2(trail_head: &Pos2D, map: &Map) -> u32 {
    let mut to_visit: Vec<Pos2D> = Vec::new();

    to_visit.push(trail_head.clone());

    let mut score = 0;

    loop {
        let current = to_visit.pop();
        if current.is_none() {
            break;
        }

        let current = current.unwrap();

        if map.get_height(&current).unwrap() == 9 {
            score += 1;
        }

        let candidates = get_next_pos_vec(&current, &map);
        for candidate in candidates {
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
    let map = init_map(input);

    let mut total_score = 0;
    for trail_head in map.trail_heads.iter() {
        let score = calculate_score2(&trail_head, &map);
        // println!("trail {:?} has score: {}", trail_head, score);
        total_score += score;
    }

    Some(total_score)
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
        assert_eq!(result, Some(81));
    }
}

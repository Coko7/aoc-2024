use std::usize;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut lines = input.lines();
        let mut robots = Vec::new();

        let size = parse_i32_vec2(lines.next().unwrap());
        for line in lines {
            let robot = Robot::from_input(line);
            robots.push(robot);
        }

        Map {
            width: size.0.try_into().unwrap(),
            height: size.1.try_into().unwrap(),
            robots,
        }
    }

    fn update_robots(&mut self, elapsed_time: u32) {
        for _ in 0..elapsed_time {
            for robot in self.robots.iter_mut() {
                robot.move_it(self.width, self.height);
            }
        }
    }

    fn is_top_left_quad(&self, position: &Pos) -> bool {
        let quad_start = (0, 0);
        let quad_end = ((self.width as i32 - 1) / 2, (self.height as i32 - 1) / 2);

        position.x >= quad_start.0
            && position.x < quad_end.0
            && position.y >= quad_start.1
            && position.y < quad_end.1
    }

    fn is_top_right_quad(&self, position: &Pos) -> bool {
        let quad_start = ((self.width as i32 + 1) / 2, 0);
        let quad_end = (self.width as i32, (self.height as i32 - 1) / 2);

        position.x >= quad_start.0
            && position.x < quad_end.0
            && position.y >= quad_start.1
            && position.y < quad_end.1
    }

    fn is_bot_right_quad(&self, position: &Pos) -> bool {
        let quad_start = ((self.width as i32 + 1) / 2, (self.height as i32 + 1) / 2);
        let quad_end = (self.width as i32, self.height as i32);

        position.x >= quad_start.0
            && position.x < quad_end.0
            && position.y >= quad_start.1
            && position.y < quad_end.1
    }

    fn is_bot_left_quad(&self, position: &Pos) -> bool {
        let quad_start = (0, (self.height as i32 + 1) / 2);
        let quad_end = ((self.width as i32 - 1) / 2, self.height as i32);

        position.x >= quad_start.0
            && position.x < quad_end.0
            && position.y >= quad_start.1
            && position.y < quad_end.1
    }

    fn get_quadrant(&self, position: &Pos) -> Option<u32> {
        if self.is_top_left_quad(position) {
            Some(0)
        } else if self.is_top_right_quad(position) {
            Some(1)
        } else if self.is_bot_right_quad(position) {
            Some(2)
        } else if self.is_bot_left_quad(position) {
            Some(3)
        } else {
            None
        }
    }

    fn count_robots_in_quadrants(&self) -> Vec<usize> {
        let mut quadrants = vec![0, 0, 0, 0];
        for robot in self.robots.iter() {
            if let Some(quad_idx) = self.get_quadrant(&robot.position) {
                let quad_idx = quad_idx as usize;
                quadrants[quad_idx] += 1;
            }
        }
        quadrants
    }

    fn get_safety_factor(&self) -> u32 {
        self.count_robots_in_quadrants()
            .iter()
            .map(|&x| x as u32)
            .fold(1, |acc, x| acc * x)
    }
}

#[derive(Debug)]
struct Robot {
    position: Pos,
    velocity: (i32, i32),
}

impl Robot {
    fn from_input(input: &str) -> Robot {
        let parts: Vec<&str> = input.split(" ").collect();

        let position = parse_i32_vec2(parts.first().unwrap());
        let position = Pos {
            x: position.0,
            y: position.1,
        };

        let velocity = parse_i32_vec2(parts.last().unwrap());

        Robot { position, velocity }
    }

    fn move_it(&mut self, map_width: usize, map_height: usize) {
        self.position.x += self.velocity.0;
        self.position.y += self.velocity.1;

        self.position.x = self.position.x.rem_euclid(map_width as i32);
        self.position.y = self.position.y.rem_euclid(map_height as i32);
    }
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_i32_vec2(input: &str) -> (i32, i32) {
    let parts: Vec<&str> = input.split(",").collect();
    let first = parts
        .first()
        .unwrap()
        .split("=")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let second = parts.last().unwrap().parse::<i32>().unwrap();
    (first, second)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_input(input);
    map.update_robots(100);
    Some(map.get_safety_factor())
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

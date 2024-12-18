use core::panic;

use advent_of_code::{Direction, Pos2D};

advent_of_code::solution!(6);

fn out_of_bounds(pos: &Pos2D, width: usize, height: usize) -> bool {
    if pos.x < 0 || pos.x >= width.try_into().unwrap() {
        return true;
    }

    if pos.y < 0 || pos.y >= height.try_into().unwrap() {
        return true;
    }

    false
}

fn get_in_front_pos(pos: &Pos2D, facing: &Direction) -> Pos2D {
    match facing {
        Direction::Up => Pos2D {
            x: pos.x,
            y: pos.y - 1,
        },
        Direction::Right => Pos2D {
            x: pos.x + 1,
            y: pos.y,
        },
        Direction::Down => Pos2D {
            x: pos.x,
            y: pos.y + 1,
        },
        Direction::Left => Pos2D {
            x: pos.x - 1,
            y: pos.y,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start_pos: Option<Pos2D> = None;

    let mut obstacles: Vec<Pos2D> = Vec::new();

    let height = input.lines().count();
    let width = input.lines().last().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            match tile {
                '.' => {}
                '#' => obstacles.push(Pos2D { x, y }),
                '^' => {
                    assert!(start_pos.is_none(), "start_pos cannot be already set!");
                    start_pos = Some(Pos2D { x, y })
                }
                c => panic!("Unknown character: {}", c),
            }
        }
    }

    assert!(start_pos.is_some(), "start_pos should have been set!");

    let start_pos = start_pos.unwrap();

    let mut current_pos = start_pos.clone();
    let mut facing = Direction::Up;

    let mut visited: Vec<Pos2D> = Vec::new();

    loop {
        if !visited
            .iter()
            .any(|p| p.x == current_pos.x && p.y == current_pos.y)
        {
            visited.push(current_pos.clone());
        }

        let in_front_pos = get_in_front_pos(&current_pos, &facing);
        if out_of_bounds(&in_front_pos, width, height) {
            break;
        }

        let blocked = obstacles
            .iter()
            .any(|p| p.x == in_front_pos.x && p.y == in_front_pos.y);

        if blocked {
            facing = Direction::rotate(&facing);
        } else {
            current_pos = in_front_pos;
        }
    }

    // println!("{:#?}", visited);
    Some(visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start_pos: Option<Pos2D> = None;

    let mut obstacles: Vec<Pos2D> = Vec::new();

    let height = input.lines().count();
    let width = input.lines().last().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            match tile {
                '.' => {}
                '#' => obstacles.push(Pos2D { x, y }),
                '^' => {
                    assert!(start_pos.is_none(), "start_pos cannot be already set!");
                    start_pos = Some(Pos2D { x, y })
                }
                c => panic!("Unknown character: {}", c),
            }
        }
    }

    assert!(start_pos.is_some(), "start_pos should have been set!");

    let start_pos = start_pos.unwrap();

    let mut facing = Direction::Up;
    let mut current_pos = start_pos.clone();

    let mut visited: Vec<Pos2D> = Vec::new();
    let mut loop_candidates = Vec::new();

    loop {
        if !visited
            .iter()
            .any(|p| p.x == current_pos.x && p.y == current_pos.y)
        {
            visited.push(current_pos.clone());
        }

        let in_front_pos = get_in_front_pos(&current_pos, &facing);
        if out_of_bounds(&in_front_pos, width, height) {
            break;
        }

        let blocked = obstacles
            .iter()
            .any(|p| p.x == in_front_pos.x && p.y == in_front_pos.y);

        if blocked {
            facing = Direction::rotate(&facing);
        } else {
            current_pos = in_front_pos;
        }
    }

    // Try only putting obstacles on positions that we would have visited initially
    for pos in visited.iter() {
        let x: i32 = pos.x.try_into().unwrap();
        let y: i32 = pos.y.try_into().unwrap();

        // let progress = idx as f32 / visited.len() as f32;
        // println!("{}%", progress * 100.0);

        if obstacles.iter().any(|p| p.x == x && p.y == y) {
            continue;
        }

        let mut facing = Direction::Up;
        let mut current_pos = start_pos.clone();

        let mut obstacles_copy = obstacles.clone();
        obstacles_copy.push(Pos2D { x, y });

        let mut blocked_history: Vec<(Pos2D, Direction)> = Vec::new();

        loop {
            let in_front_pos = get_in_front_pos(&current_pos, &facing);
            let blocked = obstacles_copy
                .iter()
                .any(|p| p.x == in_front_pos.x && p.y == in_front_pos.y);

            if blocked {
                // Already been blocked with same obstacle and same direction; must be in a
                // loop
                if blocked_history
                    .iter()
                    .any(|e| e.0.x == in_front_pos.x && e.0.y == in_front_pos.y && e.1 == facing)
                {
                    loop_candidates.push(Pos2D::new(x, y));
                    break;
                }

                blocked_history.push((in_front_pos, facing.clone()));
                facing = Direction::rotate(&facing);
            } else {
                current_pos = in_front_pos;
            }

            if out_of_bounds(&current_pos, width, height) {
                break;
            }

            // println!("{:?}", current_pos);
        }
    }

    Some(loop_candidates.len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

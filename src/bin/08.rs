advent_of_code::solution!(8);

struct Map {
    width: usize,
    height: usize,
    antennas: Vec<Antenna>,
}

struct Antenna {
    x: i32,
    y: i32,
    id: u32,
    frequency: char,
}

impl Antenna {
    fn new(id: u32, frequency: char, x: i32, y: i32) -> Self {
        Antenna {
            id,
            frequency,
            x,
            y,
        }
    }
}

fn process_map(input: &str) -> Map {
    let mut antennas = Vec::new();

    let width = input.lines().last().unwrap().chars().count();
    let height = input.lines().count();

    // Find antennas
    let mut serial = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                val if val.is_alphanumeric() => {
                    let x: i32 = x.try_into().unwrap();
                    let y: i32 = y.try_into().unwrap();
                    let antenna = Antenna::new(serial, val, x, y);
                    antennas.push(antenna);
                    serial += 1;
                }
                _ => {}
            }
        }
    }

    Map {
        width,
        height,
        antennas,
    }
}

fn in_bounds(x: i32, y: i32, map: &Map) -> bool {
    x >= 0 && x < map.width.try_into().unwrap() && y >= 0 && y < map.height.try_into().unwrap()
}

fn find_antinodes(map: &Map) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    let antennas = &map.antennas;

    for antenna in antennas.iter() {
        let resonant_antennas = antennas
            .iter()
            .filter(|a| a.frequency == antenna.frequency && a.id != antenna.id);

        for reso in resonant_antennas {
            let dx = antenna.x - reso.x;
            let dy = antenna.y - reso.y;

            let antinode = (antenna.x + dx, antenna.y + dy);
            if in_bounds(antinode.0, antinode.1, map) {
                if antinodes
                    .iter()
                    .any(|an: &(i32, i32)| an.0 == antinode.0 && an.1 == antinode.1)
                {
                    continue;
                }

                antinodes.push(antinode);
                // println!("{:?}", antinode);
            }
        }
    }

    antinodes
}

fn find_antinodes_2(map: &Map) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    let antennas = &map.antennas;

    for antenna in antennas.iter() {
        let resonant_antennas = antennas
            .iter()
            .filter(|a| a.frequency == antenna.frequency && a.id != antenna.id);

        for reso in resonant_antennas {
            let dx = antenna.x - reso.x;
            let dy = antenna.y - reso.y;

            let mut antinode = (antenna.x + dx, antenna.y + dy);
            loop {
                if in_bounds(antinode.0, antinode.1, map) {
                    if !antinodes
                        .iter()
                        .any(|an: &(i32, i32)| an.0 == antinode.0 && an.1 == antinode.1)
                    {
                        antinodes.push(antinode);
                        // println!("{:?}", antinode);
                    }
                } else {
                    break;
                }

                antinode = (antinode.0 + dx, antinode.1 + dy);
            }

            antinode = (antenna.x - dx, antenna.y - dy);
            loop {
                if in_bounds(antinode.0, antinode.1, map) {
                    if !antinodes
                        .iter()
                        .any(|an: &(i32, i32)| an.0 == antinode.0 && an.1 == antinode.1)
                    {
                        antinodes.push(antinode);
                        // println!("{:?}", antinode);
                    }
                } else {
                    break;
                }

                antinode = (antinode.0 - dx, antinode.1 - dy);
            }
        }
    }

    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let antennas = process_map(input);
    let antinodes = find_antinodes(&antennas);

    Some(antinodes.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let antennas = process_map(input);
    let antinodes = find_antinodes_2(&antennas);

    Some(antinodes.len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

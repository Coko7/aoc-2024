use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let patterns = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .collect();

    let designs = input.lines().skip(2).collect();
    (patterns, designs)
}

// 4 ideas of optimizations:
// - 1. Sort patterns by length DESC (longer patterns first)
// - 2. Eliminate combo patters (ex: a, b, ab | `ab` is a combo pattern of `a` and `b`)
// - 3. Match design against pattern instead of matching pattern against design
// - 4. Early invalidation of patterns: First check if pattern is contained within the design and
// if not, remove it entirely from the list for that design
fn can_prefix(design: &str, patterns: &Vec<&str>, cache: &mut HashMap<usize, ()>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns.iter() {
        if let Some(stripped_design) = design.strip_prefix(pattern) {
            let key = stripped_design.len();
            if cache.get(&key).is_some() {
                return false;
            }

            if can_prefix(stripped_design, &patterns, cache) {
                return true;
            }
        }
    }

    if cache.get(&design.len()).is_none() {
        cache.insert(design.len(), ());
    }

    false
}

fn find_possible_designs<'a>(patterns: &Vec<&'a str>, designs: &Vec<&'a str>) -> Vec<&'a str> {
    let mut possible_designs = vec![];
    for &design in designs.iter() {
        let mut cache: HashMap<usize, ()> = HashMap::new();
        if can_prefix(design, patterns, &mut cache) {
            possible_designs.push(design);
        }
    }
    possible_designs
}

fn filter_patterns_for_design<'a>(design: &str, patterns: &Vec<&'a str>) -> Vec<&'a str> {
    let relevant_patterns: Vec<&str> = patterns
        .iter()
        .filter(|&d| design.contains(d))
        .cloned()
        .collect();

    println!(
        "For `{}`: ({}/{}) contained [{}%]",
        design,
        relevant_patterns.len(),
        patterns.len(),
        (relevant_patterns.len() as f64 / patterns.len() as f64 * 100.) as u32,
    );

    println!("relevant patterns{:?}", relevant_patterns);
    relevant_patterns
}

fn debug_patterns<'a>(patterns: &Vec<&'a str>, designs: &Vec<&'a str>) {
    for (idx, design) in designs.iter().enumerate() {
        let contained = filter_patterns_for_design(design, patterns);

        println!(
            "{}. For `{}`: ({}/{}) contained [{}%]",
            idx + 1,
            design,
            contained.len(),
            patterns.len(),
            (contained.len() as f64 / patterns.len() as f64 * 100.) as u32,
        );
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut patterns, designs) = parse_input(input);
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let possible_designs = find_possible_designs(&patterns, &designs);
    Some(possible_designs.len())
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

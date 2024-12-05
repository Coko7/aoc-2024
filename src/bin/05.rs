advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules = Vec::new();
    let mut good_updates = Vec::new();

    let mut parsing_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split("|").collect();
            assert!(parts.len() == 2, "Parts len should be two");

            let rule: (u32, u32) = (
                parts.first().unwrap().parse().unwrap(),
                parts.last().unwrap().parse().unwrap(),
            );
            rules.push(rule);
        } else {
            let pages: Vec<u32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            let mut seen_pages: Vec<u32> = Vec::new();
            let mut is_good = true;

            'check_pages: for page in pages.iter() {
                // println!("Checking page {}", page);
                let matching_rules = rules.iter().filter(|r| r.0 == *page);

                for rule in matching_rules {
                    // println!("Have we seen {}? {}", rule.1, seen_pages.contains(&rule.1));
                    if seen_pages.contains(&rule.1) {
                        is_good = false;
                        break 'check_pages;
                    }
                }

                assert!(
                    !seen_pages.contains(&page),
                    "Page should have never been seen!"
                );
                seen_pages.push(*page);
            }

            if is_good {
                good_updates.push(pages);
            }
        }
    }

    let mut sum = 0;
    for update in good_updates.iter() {
        let middle_idx = update.len() / 2;
        if let Some(middle_page) = update.get(middle_idx) {
            sum += middle_page;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules = Vec::new();
    let mut bad_updates = Vec::new();

    let mut parsing_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split("|").collect();
            assert!(parts.len() == 2, "Parts len should be two");

            let rule: (u32, u32) = (
                parts.first().unwrap().parse().unwrap(),
                parts.last().unwrap().parse().unwrap(),
            );
            rules.push(rule);
        } else {
            let pages: Vec<u32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            let mut seen_pages: Vec<u32> = Vec::new();
            let mut is_good = true;

            'check_pages: for page in pages.iter() {
                // println!("Checking page {}", page);
                let matching_rules = rules.iter().filter(|r| r.0 == *page);

                for rule in matching_rules {
                    // println!("Have we seen {}? {}", rule.1, seen_pages.contains(&rule.1));
                    if seen_pages.contains(&rule.1) {
                        is_good = false;
                        break 'check_pages;
                    }
                }

                assert!(
                    !seen_pages.contains(&page),
                    "Page should have never been seen!"
                );
                seen_pages.push(*page);
            }

            if !is_good {
                println!("{:?}", pages);
                bad_updates.push(pages);
            }
        }
    }

    println!("Fixing");

    let mut fixed_updates = Vec::new();
    for bad_update in bad_updates.iter() {
        let mut fixed_pages: Vec<u32> = Vec::new();

        loop {
            let unfixed_pages: Vec<u32> = bad_update
                .iter()
                .filter(|page| !fixed_pages.contains(page))
                .map(|page| *page)
                .collect();

            if let Some(page) = unfixed_pages.first() {
                let pred = get_predecessor(*page, &unfixed_pages, &rules);
                if !fixed_pages.contains(&pred) {
                    fixed_pages.push(pred);
                }
            } else {
                break;
            }
        }

        println!("{:?}", fixed_pages);
        fixed_updates.push(fixed_pages);
    }

    let mut sum = 0;
    for update in fixed_updates.iter() {
        let middle_idx = update.len() / 2;
        if let Some(middle_page) = update.get(middle_idx) {
            sum += middle_page;
        }
    }

    Some(sum)
}

fn get_predecessor(page: u32, pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> u32 {
    let matching_rules: Vec<_> = rules.iter().filter(|r| r.1 == page).collect();

    for rule in matching_rules.iter() {
        if pages.contains(&rule.0) {
            return get_predecessor(rule.0, pages, rules);
        }
    }

    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

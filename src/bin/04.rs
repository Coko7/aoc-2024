advent_of_code::solution!(4);

fn is_xmas_char(c: char) -> bool {
    match c {
        'X' | 'M' | 'A' | 'S' => true,
        _ => false,
    }
}

fn get<T>(vec: &Vec<Vec<T>>, x: i32, y: i32) -> Option<&T> {
    if x < 0 {
        return None;
    }

    if y < 0 {
        return None;
    }

    let x: usize = x.try_into().unwrap();
    let y: usize = y.try_into().unwrap();

    if let Some(row) = vec.get(y) {
        return row.get(x);
    }

    None
}

fn is_mas_cross(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> bool {
    let x: i32 = x.try_into().unwrap();
    let y: i32 = y.try_into().unwrap();

    if let Some(center) = get(matrix, x, y) {
        assert!(*center == 'A');

        let nwc = get(matrix, x - 1, y - 1);
        let nec = get(matrix, x + 1, y - 1);
        let swc = get(matrix, x - 1, y + 1);
        let sec = get(matrix, x + 1, y + 1);

        let mut diag1 = String::from("");
        if let Some(nwc) = nwc {
            if let Some(sec) = sec {
                diag1.push(*nwc);
                diag1.push(*center);
                diag1.push(*sec);
            }
        }

        let mut diag2 = String::from("");
        if let Some(nec) = nec {
            if let Some(swc) = swc {
                diag2.push(*nec);
                diag2.push(*center);
                diag2.push(*swc);
            }
        }

        if is_mas(&diag1) && is_mas(&diag2) {
            return true;
        }
    }

    false
}

fn get_words_originating_from(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> Vec<String> {
    let mut res = Vec::new();

    let x: i32 = x.try_into().unwrap();
    let y: i32 = y.try_into().unwrap();

    // Horizontal Forward
    let c1 = get(matrix, x, y);
    if let Some(c1) = c1 {
        let c2 = get(matrix, x + 1, y);
        if let Some(c2) = c2 {
            let c3 = get(matrix, x + 2, y);
            if let Some(c3) = c3 {
                let c4 = get(matrix, x + 3, y);
                if let Some(c4) = c4 {
                    let mut word = String::from("");
                    word.push(*c1);
                    word.push(*c2);
                    word.push(*c3);
                    word.push(*c4);

                    res.push(word);
                }
            }
        }
    }

    // Vertical Forward
    let c1 = get(matrix, x, y);
    if let Some(c1) = c1 {
        let c2 = get(matrix, x, y + 1);
        if let Some(c2) = c2 {
            let c3 = get(matrix, x, y + 2);
            if let Some(c3) = c3 {
                let c4 = get(matrix, x, y + 3);
                if let Some(c4) = c4 {
                    let mut word = String::from("");
                    word.push(*c1);
                    word.push(*c2);
                    word.push(*c3);
                    word.push(*c4);

                    res.push(word);
                }
            }
        }
    }

    // Diagonal South-West
    let c1 = get(matrix, x, y);
    if let Some(c1) = c1 {
        let c2 = get(matrix, x - 1, y + 1);
        if let Some(c2) = c2 {
            let c3 = get(matrix, x - 2, y + 2);
            if let Some(c3) = c3 {
                let c4 = get(matrix, x - 3, y + 3);
                if let Some(c4) = c4 {
                    let mut word = String::from("");
                    word.push(*c1);
                    word.push(*c2);
                    word.push(*c3);
                    word.push(*c4);

                    res.push(word);
                }
            }
        }
    }

    // Diagonal South-East
    let c1 = get(matrix, x, y);
    if let Some(c1) = c1 {
        let c2 = get(matrix, x + 1, y + 1);
        if let Some(c2) = c2 {
            let c3 = get(matrix, x + 2, y + 2);
            if let Some(c3) = c3 {
                let c4 = get(matrix, x + 3, y + 3);
                if let Some(c4) = c4 {
                    let mut word = String::from("");
                    word.push(*c1);
                    word.push(*c2);
                    word.push(*c3);
                    word.push(*c4);

                    res.push(word);
                }
            }
        }
    }

    res
}

fn is_mas(word: &str) -> bool {
    match word.to_uppercase().as_str() {
        "MAS" | "SAM" => true,
        _ => false,
    }
}

fn is_xmas(word: &str) -> bool {
    match word.to_uppercase().as_str() {
        "XMAS" | "SAMX" => true,
        _ => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }

    let mut res = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if !is_xmas_char(*char) {
                continue;
            }

            let words = get_words_originating_from(x, y, &matrix);
            for word in words {
                if is_xmas(&word) {
                    res = res + 1;
                }
            }
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }

    let mut res = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != 'A' {
                continue;
            }

            if is_mas_cross(x, y, &matrix) {
                res = res + 1;
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

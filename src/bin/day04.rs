use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let file_input = fs::read_to_string(INPUT_FILE)?;
    println!("Result = {}", solve_part_one(&file_input));
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let file_input = fs::read_to_string(INPUT_FILE)?;
    let result2 = solve_part_two(&file_input);
    println!("Result = {}", result2);
    //endregion

    Ok(())
}

fn solve_part_one(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut x_chars: Vec<(i32, i32)> = Vec::new();

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (y_idx, row) in input.lines().enumerate() {
        for (x_idx, col) in row.chars().enumerate() {
            grid.insert((x_idx as i32, y_idx as i32), col);
            if col == 'X' {
                x_chars.push((x_idx as i32, y_idx as i32));
            }
        }
    }

    let search_term = "MAS";
    let mut sum = 0;

    for &(start_x, start_y) in x_chars.iter() {
        for &(dx, dy) in directions.iter() {
            let mut matched = true;
            for (factor, search_char) in search_term.chars().enumerate() {
                let new_x: i32 = start_x + dx * (factor as i32 + 1);
                let new_y: i32 = start_y + dy * (factor as i32 + 1);

                if grid.get(&(new_x, new_y)) != Some(&search_char) {
                    matched = false;
                    break;
                }
            }
            if matched {
                sum += 1;
            }
        }
    }
    sum
}

fn solve_part_two(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut a_chars: Vec<(i32, i32)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    for (y_idx, row) in input.lines().enumerate() {
        for (x_idx, col) in row.chars().enumerate() {
            grid.insert((x_idx as i32, y_idx as i32), col);
            if col == 'A' {
                a_chars.push((x_idx as i32, y_idx as i32));
            }
        }
    }
    let mut sum = 0;
    for &(start_x, start_y) in a_chars.iter() {
        let mut m_matches = Vec::new();
        let mut a_matches = Vec::new();
        for &(dx, dy) in directions.iter() {
            let new_x: i32 = start_x + dx;
            let new_y: i32 = start_y + dy;

            if grid.get(&(new_x, new_y)) == Some(&'M') {
                m_matches.push((dx, dy));
            }
            else if grid.get(&(new_x, new_y)) == Some(&'S') {
                a_matches.push((dx, dy));
            }
        }

        println!("results: {:?} {:?}", a_matches, m_matches);

        if m_matches.len() == 2 && a_matches.len() == 2 {
            if m_matches[0].0 + m_matches[0].1 + a_matches[0].0 + a_matches[0].1 == 0 {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST), 9);
    }
}

use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    let file_input = fs::read_to_string(INPUT_FILE)?;

    println!("=== Part 1 ===");
    println!("Result = {}", solve_part_one(&file_input));

    println!("\n=== Part 2 ===");
    println!("Result = {}", solve_part_two(&file_input));

    Ok(())
}

fn solve_part_one(input: &str) -> i32 {
    let mut start_coords: Option<(i32, i32)> = None;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut visited_fields: HashMap<(i32, i32), char> = HashMap::new();
    for (y_idx, row) in input.lines().enumerate() {
        for (x_idx, col) in row.chars().enumerate() {
            grid.insert((x_idx as i32, y_idx as i32), col);
            if col == '^' {
                start_coords = Some((x_idx as i32, y_idx as i32));
            }
        }
    }

    if let Some(start) = start_coords {
        let mut current_field = start;
        let mut current_direction_index = 0;
        let mut next_field = add_coords(&current_field, &directions[current_direction_index]);
        println!("Will start on {:?}", start);
        visited_fields.insert(start, 'X');

        while let Some(&value) = grid.get(&next_field) {
            if value != '#' {
                current_field = next_field;
                if !visited_fields.contains_key(&current_field) {
                    visited_fields.insert(current_field, 'X');
                }
                next_field = add_coords(&current_field, &directions[current_direction_index]);
            } else if value == '#' {
                current_direction_index = (current_direction_index + 1) % directions.len();
                next_field = add_coords(&current_field, &directions[current_direction_index]);
            }
        }
    }
    visited_fields.len() as i32
}

fn add_coords(tuple1: &(i32, i32), tuple2: &(i32, i32)) -> (i32, i32) {
    ((tuple1.0 + tuple2.0), (tuple1.1 + tuple2.1))
}

fn solve_part_two(input: &str) -> usize {
    let mut start_coords: Option<(i32, i32)> = None;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut grid = HashMap::new();

    for (y_idx, row) in input.lines().enumerate() {
        for (x_idx, col) in row.chars().enumerate() {
            grid.insert((x_idx as i32, y_idx as i32), col);
            if col == '^' {
                start_coords = Some((x_idx as i32, y_idx as i32));
            }
        }
    }

    let start_coords = start_coords.expect("Starting position not found.");

    let barricatable_cells: Vec<(i32, i32)> = grid
        .iter()
        .filter(|(_, &value)| value != '^' && value != '#')
        .map(|(&coords, _)| coords)
        .collect();

    let mut ends_in_loop = |barrier: (i32, i32)| -> bool {
        let mut visited = HashSet::new();
        let mut current = start_coords;
        let mut direction_idx = 0;

        loop {
            if visited.contains(&(current, direction_idx)) {
                return true;
            }

            visited.insert((current, direction_idx));

            let next = add_coords(&current, &directions[direction_idx]);
            let value = if next == barrier {
                Some('#')
            } else {
                grid.get(&next).copied()
            };
            match value {
                Some('.') | Some('^') => current = next,
                Some('#') => direction_idx = (direction_idx + 1) % directions.len(),
                _ => break,
            }
        }
        false
    };
    barricatable_cells
        .iter()
        .filter(|&cell| ends_in_loop(*cell))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST), 41);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST), 6);
    }
}

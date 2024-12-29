use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

type Coordinates = (i32, i32);

fn main() -> Result<()> {
    start_day(DAY);

    let file_input = fs::read_to_string(INPUT_FILE)?;

    println!("=== Part 1 ===");
    println!("Result = {}", solve_part_one(&file_input));

    println!("\n=== Part 2 ===");
    println!("Result = {}", solve_part_two(&file_input));

    Ok(())
}

fn solve_part_one(input: &str) -> usize {
    let mut antinodes: HashSet<Coordinates> = HashSet::new();
    let mut grid_positions: HashSet<Coordinates> = HashSet::new();
    let mut antennas: HashMap<char, Vec<Coordinates>> = HashMap::new();

    for (y_idx, row) in input.lines().enumerate() {
        for (x_idx, col) in row.chars().enumerate() {
            grid_positions.insert((x_idx as i32, y_idx as i32));
            if col != '.' {
                antennas
                    .entry(col)
                    .or_insert_with(Vec::new)
                    .push((x_idx as i32, y_idx as i32));
            }
        }
    }

    let mut get_antinodes = |list: &Vec<Coordinates>| {
        let antinode = |first: Coordinates, second: Coordinates| -> Coordinates {
            (
                first.0 + 2 * (second.0 - first.0),
                first.1 + 2 * (second.1 - first.1),
            )
        };

        for i in 0..list.len() {
            for j in i + 1..list.len() {
                let a1 = antinode(list[i], list[j]);
                let a2 = antinode(list[j], list[i]);

                // check if in bounds of grid
                if let Some(_) = grid_positions.get(&a1) {
                    if !antinodes.contains(&a1) {
                        antinodes.insert(a1);
                    }
                }
                if let Some(_) = grid_positions.get(&a2) {
                    if !antinodes.contains(&a2) {
                        antinodes.insert(a2);
                    }
                }
            }
        }
    };
    antennas.iter().for_each(|content| get_antinodes(content.1));
    antinodes.len()
}

fn solve_part_two(input: &str) -> usize {
    let mut antinodes: HashSet<Coordinates> = HashSet::new();
    let mut grid_dimensions = (0, 0);
    let mut antennas: HashMap<char, Vec<Coordinates>> = HashMap::new();

    for (y_idx, row) in input.lines().enumerate() {
        grid_dimensions.0 = y_idx;
        for (x_idx, col) in row.chars().enumerate() {
            grid_dimensions.1 = x_idx;
            if col != '.' {
                antennas
                    .entry(col)
                    .or_insert_with(Vec::new)
                    .push((x_idx as i32, y_idx as i32));
            }
        }
    }

    let antinode_vector =
        |first: Coordinates, second: Coordinates| ((second.0 - first.0), (second.1 - first.1));

    let mut get_antinodes = |list: &Vec<Coordinates>| {
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                // search first direction
                find_antinodes_recursive(
                    list[i],
                    1,
                    &grid_dimensions,
                    antinode_vector(list[i], list[j]),
                    &mut antinodes,
                );

                // search second direction
                find_antinodes_recursive(
                    list[j],
                    1,
                    &grid_dimensions,
                    antinode_vector(list[j], list[i]),
                    &mut antinodes,
                );
            }
        }
    };
    antennas.iter().for_each(|content| get_antinodes(content.1));
    antinodes.len()
}

fn find_antinodes_recursive(
    start: Coordinates,
    factor: i32,
    dimensions: &(usize, usize),
    vector: Coordinates,
    antinodes: &mut HashSet<Coordinates>,
) {
    let next = (start.0 + factor * vector.0, start.1 + factor * vector.1);
    if next.0 <= dimensions.0 as i32 && next.0 >= 0 && next.1 <= dimensions.1 as i32 && next.1 >= 0
    {
        if !antinodes.contains(&next) {
            antinodes.insert(next);
        }
        find_antinodes_recursive(start, factor + 1, dimensions, vector, antinodes);
    }
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST), 14);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST), 34);
    }
}

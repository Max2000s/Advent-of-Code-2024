use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let result = solve_part_one(TEST);

    // TODO: Set the expected answer for the test input
    assert_eq!(11, result);
    println!("Test worked: {}", result);

    let file_input = fs::read_to_string(INPUT_FILE)?;
    let result = solve_part_one(&file_input);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let result = solve_part_two(TEST);

    assert_eq!(31, result);
    println!("Test worked: {}", result);

    let file_input = fs::read_to_string(INPUT_FILE)?;
    let result2 = solve_part_two(&file_input);
    println!("Result = {}", result2);

    //endregion

    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let lines = input.lines();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        left.push(parts[0].parse::<u32>().unwrap());
        right.push(parts[1].parse::<u32>().unwrap());
    }
    left.sort();
    right.sort();
    let mut sum: u32 = 0;
    for index in 0..left.len() {
        sum += right[index].abs_diff(left[index]);
    }
    sum
}

fn solve_part_two(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let lines = input.lines();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        left.push(parts[0].parse::<u32>().unwrap());
        right.push(parts[1].parse::<u32>().unwrap());
    }
    let mut sum = 0;
    for left_num in &left {
        let mut factor = 0;
        for right_num in &right {
            if left_num == right_num {
                factor += 1;
            }
        }
        sum += factor * left_num;
    }
    sum
}

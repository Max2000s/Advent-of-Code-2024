use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let result = solve_part_one(TEST);

    // TODO: Set the expected answer for the test input
    assert_eq!(2, result);
    println!("Test worked: {}", result);

    let file_input = fs::read_to_string(INPUT_FILE)?;
    let result = solve_part_one(&file_input);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let result = solve_part_two(TEST);

    assert_eq!(4, result);
    println!("Test worked: {}", result);

    let file_input = fs::read_to_string(INPUT_FILE)?;
    let result2 = solve_part_two(&file_input);
    println!("Result = {}", result2);

    //endregion

    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x
            .split_whitespace()
            .map(|y| y
                .parse::<i32>()
                .unwrap())
            .collect::<Vec<_>>())
        .filter(|valid| is_valid(valid))
        .count() as u32
}

fn solve_part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x
            .split_whitespace()
            .map(|y| y
                .parse::<i32>()
                .unwrap())
            .collect::<Vec<_>>())
        .filter(|valid| check_valid_report_two(valid))
        .count() as u32
}

fn check_valid_report_two(report: &Vec<i32>) -> bool {
    if is_valid(report) {
        true
    } else {
        for i in 0..report.len() {
            let mut temp_report = report.clone();
            temp_report.remove(i);
            if is_valid(&temp_report) {
                return true;
            }
            continue;
        }
        false
    }
}

fn is_valid(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true; // return if only one entry is available
    }

    let is_increasing = report[0] < report[1];

    for i in 1..report.len() {
        let difference = report[i].abs_diff(report[i - 1]);
        if is_increasing {
            if report[i - 1] > report[i] || difference < 1 || difference > 3 {
                return false;
            }
        } else {
            if report[i - 1] < report[i] || difference < 1 || difference > 3 {
                return false;
            }
        }
    }
    true
}
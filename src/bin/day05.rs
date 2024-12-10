use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
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
    let rules: Vec<Vec<i32>> = input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|rule| {
            rule.split("|")
                .filter_map(|num| num.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();
    let entries: Vec<Vec<i32>> = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|entry| {
            entry
                .split(",")
                .filter_map(|num| num.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();

    let mut sum: i32 = 0;

    for entry in &entries {
        let mut valid = true;
        for rule in &rules {
            if let (Some(&rule_0), Some(&rule_1)) = (rule.get(0), rule.get(1)) {
                if entry.contains(&rule_0) && entry.contains(&rule_1) {
                    let index_first = entry.iter().position(|&r| r == rule_0).unwrap();
                    let index_second = entry.iter().position(|&r| r == rule_1).unwrap();
                    if index_first > index_second {
                        valid = false;
                        break;
                    }
                }
            }
        }
        println!("entry is valid: {}", valid);
        println!("entry has {} fields", entry.len());
        if valid {
            if let Some(&middle_field) = entry.get(entry.len() / 2) {
                println!("taking field {}", middle_field);
                sum += middle_field;
            }
        }
    }
    sum
}

fn solve_part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST), 143);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST), 123);
    }
}

use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");



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
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let multiplications: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut sum = 0;
    for multiplication in multiplications {
        sum += calculate(multiplication);
    }
    sum
}

fn calculate(expression: &str) -> i32 {
    let re = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
    let factors_regex: Vec<&str>= re.find_iter(expression).map(|m| m.as_str()).collect();
    let factors: Vec<i32> = factors_regex[0]
        .split(",")
        .map(|y| y
            .parse::<i32>()
            .unwrap())
        .collect();
    factors[0]* factors[1]
}

fn solve_part_two(input: &str) -> i32 {
    // split the "do()"
    let mut sum = 0;
    let dos = input.split("do()").collect::<Vec<&str>>();
    for do_part in dos {
        let donts = do_part.split("don't()").collect::<Vec<&str>>();

        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let multiplications: Vec<&str> = re.find_iter(donts[0]).map(|m| m.as_str()).collect();
        for multiplication in multiplications {
            sum += calculate(multiplication);
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
        const TEST: &str = "\
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        ";
        assert_eq!(solve_part_one(TEST), 161);
    }

    #[test]
    fn test_part_two() {
        const TEST: &str = "\
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        ";
        assert_eq!(solve_part_two(TEST), 48);
    }
}
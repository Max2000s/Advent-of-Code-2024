use advent_of_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/day", DAY, ".txt");

type Components = Vec<i64>;
type Equation = (i64, Components);

fn main() -> Result<()> {
    start_day(DAY);

    let file_input = fs::read_to_string(INPUT_FILE)?;

    println!("=== Part 1 ===");
    println!("Result = {}", solve_part_one(&file_input));

    println!("\n=== Part 2 ===");
    println!("Result = {}", solve_part_two(&file_input));

    Ok(())
}

fn solve_part_one(input: &str) -> i64 {
    // get the values from input
    let equations: Vec<Equation> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let result = parts.next()?.trim().parse::<i64>().ok()?;
            let components = parts
                .next()?
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i64>().ok())
                .collect::<Option<Vec<i64>>>()?;
            Some((result, components))
        })
        .collect();

    // apply a function on each equation in order to check valid lines
    equations
        .iter()
        .filter(|&equation| recursive_valid_equation(0, 0, &equation))
        .map(|equation| equation.0)
        .sum()
}

fn recursive_valid_equation(index: usize, result: i64, equation: &Equation) -> bool {
    if index == equation.1.len() {
        return result == equation.0;
    }
    let plus = recursive_valid_equation(index + 1, result + equation.1[index], equation);
    let mult = recursive_valid_equation(index + 1, result * equation.1[index], equation);
    return plus || mult;
}

fn recursive_valid_equation_part_two(index: usize, result: i64, equation: &Equation) -> bool {
    if index == equation.1.len() {
        return result == equation.0;
    }
    let concatted = format!("{}{}", result, equation.1[index]);
    let concat =
        recursive_valid_equation_part_two(index + 1, concatted.parse().ok().unwrap(), equation);
    let plus = recursive_valid_equation_part_two(index + 1, result + equation.1[index], equation);
    let mult = recursive_valid_equation_part_two(index + 1, result * equation.1[index], equation);
    return plus || mult || concat;
}

fn solve_part_two(input: &str) -> i64 {
    let equations: Vec<Equation> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let result = parts.next()?.trim().parse::<i64>().ok()?;
            let components = parts
                .next()?
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i64>().ok())
                .collect::<Option<Vec<i64>>>()?;
            Some((result, components))
        })
        .collect();

    equations
        .iter()
        .filter(|&equation| recursive_valid_equation_part_two(0, 0, &equation))
        .map(|equation| equation.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
    ";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(TEST), 3749);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(TEST), 11387);
    }
}

use std::cmp::max;

use itertools::{repeat_n, Itertools};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    operands: Vec<i64>,
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Unable to read file")
}

fn parse_input(input: String) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (test_value, operands) = line.split_once(": ").unwrap();
            Equation {
                test_value: test_value.parse().unwrap(),
                operands: operands
                    .split(" ")
                    .map(|operand| operand.parse().unwrap())
                    .collect(),
            }
        })
        .collect::<Vec<Equation>>()
}

fn get_combinations(operators: Vec<Operator>, count: usize) -> Vec<Vec<Operator>> {
    if count == 0 {
        panic!("Invalid count");
    }
    repeat_n(operators, count)
        .multi_cartesian_product()
        .collect()
}

fn check_equation(equation: Equation, operators: Vec<Operator>) -> i64 {
    let mut combinations = get_combinations(operators, equation.operands.len() - 1);
    for combination in combinations.iter_mut() {
        let mut equation = equation.clone();
        while equation.operands.len() > 1 {
            match combination[0] {
                Operator::Add => {
                    let result = equation.operands[0] + equation.operands[1];
                    equation.operands[0] = result;
                }
                Operator::Mul => {
                    let result = equation.operands[0] * equation.operands[1];
                    equation.operands[0] = result;
                }
                Operator::Concat => {
                    let a = equation.operands[0];
                    let b = equation.operands[1];
                    let result = a * 10i64.pow(b.ilog10() + 1) + b;
                    equation.operands[0] = result;
                }
            }
            equation.operands.remove(1);
            combination.remove(0);
        }

        if equation.operands[0] == equation.test_value {
            return equation.operands[0];
        }
    }
    0
}

fn part01() {
    let input = get_input("input.txt");
    let equations = parse_input(input);

    let result: i64 = equations
        .par_iter()
        .cloned()
        .map(|equation| check_equation(equation, vec![Operator::Add, Operator::Mul]))
        .sum();
    println!("Part 01: {}", result);
}

fn part02() {
    let input = get_input("input.txt");
    let equations = parse_input(input);

    let result: i64 = equations
        .par_iter()
        .cloned()
        .map(|equation| {
            check_equation(
                equation,
                vec![Operator::Add, Operator::Mul, Operator::Concat],
            )
        })
        .sum();
    println!("Part 02: {}", result);
}

fn main() {
    part01();
    part02();
}

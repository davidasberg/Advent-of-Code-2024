use std::{collections::HashMap, hash::Hash, ops::Deref};

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink_stones(stones: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    for &stone in stones {
        let digits = stone.to_string().chars().collect::<Vec<char>>();
        if stone == 0 {
            result.push(1);
        } else if digits.len() % 2 == 0 {
            let left = digits[0..digits.len() / 2].iter().collect::<String>();
            let right = digits[digits.len() / 2..].iter().collect::<String>();

            let left_val = left.parse::<u64>().unwrap();
            let right_val = right.parse::<u64>().unwrap();
            result.push(left_val);
            result.push(right_val);
        } else {
            result.push(stone * 2024);
        }
    }
    result
}

fn blink_stones_batched(stones_map: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();
    for &stone in stones_map.keys() {
        let digits = stone.to_string().chars().collect::<Vec<char>>();
        if stone == 0 {
            let count = result.entry(1).or_insert(0);
            *count += stones_map.get(&stone).unwrap();
        } else if digits.len() % 2 == 0 {
            let left = digits[0..digits.len() / 2].iter().collect::<String>();
            let right = digits[digits.len() / 2..].iter().collect::<String>();

            let left_val = left.parse::<u64>().unwrap();
            let right_val = right.parse::<u64>().unwrap();

            {
                let left_count = result.entry(left_val).or_insert(0);
                *left_count += stones_map.get(&stone).unwrap();
            }
            {
                let right_count = result.entry(right_val).or_insert(0);
                *right_count += stones_map.get(&stone).unwrap();
            }
        } else {
            let count = result.entry(stone * 2024).or_insert(0);
            *count += stones_map.get(&stone).unwrap();
        }
    }

    result
}

fn part01() {
    let input = get_input("input.txt");
    let mut stones = parse_input(&input);

    for _ in 0..25 {
        stones = blink_stones(&stones);
    }

    println!("{}", stones.len());
}

fn part02() {
    let input = get_input("input.txt");
    let stones = parse_input(&input);
    let mut map: HashMap<u64, u64> = stones.iter().fold(HashMap::new(), |mut map, &stone| {
        *map.entry(stone).or_insert(0) += 1;
        map
    });
    for _ in 0..75 {
        map = blink_stones_batched(&map)
    }

    let total_stones = map.iter().fold(0, |acc, (_, &val)| acc + val);
    println!("{}", total_stones);
}

fn main() {
    part01();
    part02();
}

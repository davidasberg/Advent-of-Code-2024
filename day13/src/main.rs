use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    ops::{Add, Mul},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IVec2 {
    x: i64,
    y: i64,
}

impl IVec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<i64> for IVec2 {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl Mul<i64> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug)]
struct ClawMachine {
    a_claw: IVec2,
    b_claw: IVec2,
    prize_pos: IVec2,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct ButtonPresses {
    a: i64,
    b: i64,
}

impl PartialOrd for ButtonPresses {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ButtonPresses {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_total = self.a * 3 + self.b;
        let other_total = other.a * 3 + other.b;
        self_total.cmp(&other_total)
    }
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn parse_input(input: &str, error_correction: bool) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|machine| {
            let lines: Vec<&str> = machine.lines().collect::<Vec<&str>>();
            let a_claw = lines[0]
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let a_claw = IVec2::new(
                a_claw.0.split_once("+").unwrap().1.parse().unwrap(),
                a_claw.1.split_once("+").unwrap().1.parse().unwrap(),
            );

            let b_claw = lines[1]
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let b_claw = IVec2::new(
                b_claw.0.split_once("+").unwrap().1.parse().unwrap(),
                b_claw.1.split_once("+").unwrap().1.parse().unwrap(),
            );

            let prize_pos = lines[2]
                .split_once(": ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();

            let prize_pos = IVec2::new(
                prize_pos.0.split_once("=").unwrap().1.parse().unwrap(),
                prize_pos.1.split_once("=").unwrap().1.parse().unwrap(),
            ) + if error_correction { 10000000000000 } else { 0 };

            ClawMachine {
                a_claw,
                b_claw,
                prize_pos,
            }
        })
        .collect()
}

fn get_minimal_cost(machine: &ClawMachine) -> Option<i64> {
    // px = i*ax + j*bx
    // py = i*ay + j*by
    // A = [ax bx] x = [i]
    //     [ay by]     [j]
    // Ax = C = [px]
    //          [py]
    //
    // D = |Ax| = ax*by - ay*bx
    // Di = px*by - py*bx
    // Dj = ax*py - ay*px
    //
    // i = Di/D
    // j = Dj/D
    // answer = 3*i + j

    let ax = machine.a_claw.x;
    let ay = machine.a_claw.y;
    let bx = machine.b_claw.x;
    let by = machine.b_claw.y;
    let px = machine.prize_pos.x;
    let py = machine.prize_pos.y;

    let d = ax * by - ay * bx;
    let di = px * by - py * bx;
    let dj = py * ax - px * ay;

    if di % d == 0 && dj % d == 0 {
        let i = di / d;
        let j = dj / d;
        Some(3 * i + j)
    } else {
        None
    }
}

fn part01() {
    let input = get_input("input.txt");
    let machines = parse_input(&input, false);

    let mut total_cost = 0;
    for machine in machines {
        let minimal_cost = get_minimal_cost(&machine);
        total_cost += minimal_cost.unwrap_or(0);
    }

    println!("Total cost: {}", total_cost);
}

fn part02() {
    let input = get_input("input.txt");
    let machines = parse_input(&input, true);

    let mut total_cost = 0;
    for machine in machines {
        let minimal_cost = get_minimal_cost(&machine);
        total_cost += minimal_cost.unwrap_or(0);
    }

    println!("Total cost: {}", total_cost);
}

fn main() {
    part01();
    part02();
}

use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IVec2 {
    x: i64,
    y: i64,
}

impl IVec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn wrap(self, max: IVec2) -> Self {
        let mut result = self;
        if self.x < 0 {
            result.x += max.x;
        }
        if self.x >= max.x {
            result.x -= max.x;
        }
        if self.y < 0 {
            result.y += max.y;
        }
        if self.y >= max.y {
            result.y -= max.y;
        }
        result
    }
}

impl Add for IVec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i64> for IVec2 {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(" ").unwrap();
            let p = x.split_once("=").unwrap().1.split_once(",").unwrap();
            let v = y.split_once("=").unwrap().1.split_once(",").unwrap();
            Robot {
                position: IVec2::new(p.0.parse().unwrap(), p.1.parse().unwrap()),
                velocity: IVec2::new(v.0.parse().unwrap(), v.1.parse().unwrap()),
            }
        })
        .collect()
}

fn tick_robots(robots: &mut [Robot], map_size: IVec2) {
    for robot in robots {
        let new_pos = IVec2::new(
            robot.position.x + robot.velocity.x,
            robot.position.y + robot.velocity.y,
        )
        .wrap(map_size);
        robot.position = new_pos;
    }
}

const MAP_SIZE: IVec2 = IVec2 { x: 101, y: 103 };

fn part01() {
    let input = get_input("input.txt");
    let mut robots = parse_input(&input);

    for _ in 0..100 {
        tick_robots(&mut robots, MAP_SIZE);
    }

    // if any robot has a negative position or is outside the map, print it
    for robot in &robots {
        if robot.position.x < 0 || robot.position.y < 0 {
            println!("Negative position: {:?}", robot);
        }
        if robot.position.x >= MAP_SIZE.x || robot.position.y >= MAP_SIZE.y {
            println!("Outside map: {:?}", robot);
        }
    }

    // count the number of robots in each quadrant
    let mut count = [0; 4];
    for robot in &robots {
        // if exactly in middle (horizontally or vertically), it is excluded
        if robot.position.x == MAP_SIZE.x / 2 || robot.position.y == MAP_SIZE.y / 2 {
            continue;
        }
        let x = if robot.position.x > MAP_SIZE.x / 2 {
            1
        } else {
            0
        };
        let y = if robot.position.y > MAP_SIZE.y / 2 {
            1
        } else {
            0
        };
        count[x + y * 2] += 1;
    }

    let product = count[0] * count[1] * count[2] * count[3];
    println!("Product: {}", product);
}

fn part02() {
    let input = get_input("input.txt");
    let mut robots = parse_input(&input);

    let mut i = 0;
    loop {
        i += 1;
        tick_robots(&mut robots, MAP_SIZE);
        let unique_positions: HashSet<IVec2> = robots.iter().map(|r| r.position).collect();
        if unique_positions.len() == robots.len() {
            println!("Tree found at {}", i);

            for y in 0..MAP_SIZE.y {
                for x in 0..MAP_SIZE.x {
                    let pos = IVec2::new(x, y);
                    let count = robots
                        .iter()
                        .filter(|r| r.position.x == pos.x && r.position.y == pos.y)
                        .count();
                    if count > 0 {
                        print!("{}", count);
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            break;
        }
    }
}

fn main() {
    part01();
    part02();
}

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct IVec2d {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for IVec2d {
    fn from((x, y): (i32, i32)) -> Self {
        IVec2d { x, y }
    }
}

impl std::ops::Add for IVec2d {
    type Output = IVec2d;

    fn add(self, other: IVec2d) -> IVec2d {
        IVec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for IVec2d {
    type Output = IVec2d;

    fn sub(self, other: IVec2d) -> IVec2d {
        IVec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Antenna {
    frequency_id: i32,
    pos: IVec2d,
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> (Vec<Antenna>, IVec2d) {
    let mut frequencies = HashMap::new();
    let map_size = (
        input.lines().next().unwrap().chars().count() as i32,
        input.lines().count() as i32,
    )
        .into();
    let antennas: Vec<Antenna> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '.' {
                        return None;
                    }

                    let id = frequencies.get(&c);
                    let id = match id {
                        Some(id) => *id,
                        None => {
                            let id = frequencies.len() as i32;
                            frequencies.insert(c, id);
                            id
                        }
                    };

                    Some(Antenna {
                        frequency_id: id,
                        pos: (x as i32, y as i32).into(),
                    })
                })
                .collect::<Vec<Antenna>>()
        })
        .collect();

    (antennas, map_size)
}

fn check_antinodes(antennas: &Vec<Antenna>, map_size: IVec2d, simple: bool) -> i32 {
    let antinodes = antennas
        .iter()
        .cartesian_product(antennas)
        .filter_map(|(a1, a2)| {
            if a1 == a2 || a1.frequency_id != a2.frequency_id {
                return None;
            }

            let mut anti_nodes = vec![];

            let is_inside_map =
                |pos: IVec2d| pos.x >= 0 && pos.y >= 0 && pos.x < map_size.x && pos.y < map_size.y;

            let a1_a2 = a2.pos - a1.pos;
            if simple {
                let anti_node = a2.pos + a1_a2;
                if is_inside_map(anti_node) {
                    anti_nodes.push(anti_node);
                }
            } else {
                let mut anti_node = a2.pos;
                while is_inside_map(anti_node) {
                    anti_nodes.push(anti_node);
                    anti_node = anti_node + a1_a2;
                }
            }
            if anti_nodes.is_empty() {
                return None;
            }
            Some(anti_nodes)
        })
        .flatten()
        .unique()
        .collect::<Vec<IVec2d>>();

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            if antinodes.contains(&(x, y).into()) {
                print!("#");
            } else if let Some(a) = antennas.iter().find(|a| a.pos == (x, y).into()) {
                print!("{}", a.frequency_id);
            } else {
                print!(".");
            }
        }
        println!();
    }

    antinodes.len() as i32
}

fn part01() {
    let input = get_input("example.txt");
    let (antennas, map_size) = parse_input(&input);
    let antinode_count = check_antinodes(&antennas, map_size, true);

    println!("part01: {}", antinode_count);
}

fn part02() {
    let input = get_input("input.txt");
    let (antennas, map_size) = parse_input(&input);
    let antinode_count = check_antinodes(&antennas, map_size, false);

    println!("part02: {}", antinode_count);
}

fn main() {
    part01();
    part02();
}

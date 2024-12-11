use std::{fmt, ops::Deref};

#[derive(Debug)]
struct Map(Vec<Vec<u32>>);

impl Deref for Map {
    type Target = Vec<Vec<u32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct IVec2D {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for IVec2D {
    fn from((x, y): (i32, i32)) -> Self {
        IVec2D { x, y }
    }
}

struct TrailHead {
    pos: IVec2D,
    score: Score,
}

type Score = u32;

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.iter() {
            for col in row.iter() {
                write!(f, "{}", col);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: String) -> Map {
    Map(input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect())
}

fn dfs(x: i32, y: i32, map: &Map) -> 
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let mut neighbors = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    stack.append(&mut neighbors);

    loop {
        if stack.is_empty() {
            break;
        }
    }
}

fn find_all_trailhead_scores(map: &Map) {
    let mut trail_heads: Vec<TrailHead> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let score = dfs(x as i32, y as i32, map);

            trail_heads.push(TrailHead {
                pos: (x as i32, y as i32).into(),
                score,
            });
        }
    }
}

fn part01() {
    let input = get_input("example.txt");
    let map = parse_input(input);
    println!("{}", map);
}

fn part02() {}

fn main() {
    part01();
}

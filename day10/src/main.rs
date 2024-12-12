use std::{fmt, ops::Deref};

#[derive(Debug)]
struct Map(Vec<Vec<i32>>);

impl Map {
    fn width(&self) -> usize {
        self[0].len()
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32
    }

    fn score(&self, x: i32, y: i32) -> i32 {
        self[x as usize][y as usize]
    }
}

impl Deref for Map {
    type Target = Vec<Vec<i32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct IVec2D {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for IVec2D {
    fn from((x, y): (i32, i32)) -> Self {
        IVec2D { x, y }
    }
}

#[derive(Debug)]
struct TrailHead {
    pos: IVec2D,
    score: i32,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
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
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect())
}

fn get_neighbors(x: i32, y: i32, map: &Map) -> Vec<IVec2D> {
    let mut neighbors: Vec<IVec2D> = Vec::new();
    if map.within_bounds(x - 1, y) {
        neighbors.push((x - 1, y).into());
    }
    if map.within_bounds(x + 1, y) {
        neighbors.push((x + 1, y).into());
    }
    if map.within_bounds(x, y - 1) {
        neighbors.push((x, y - 1).into());
    }
    if map.within_bounds(x, y + 1) {
        neighbors.push((x, y + 1).into());
    }
    neighbors
}

fn dfs(x: i32, y: i32, map: &Map, unique: bool) -> i32 {
    let mut total_score = 0;

    let mut stack: Vec<IVec2D> = Vec::new();
    let mut visited: Vec<IVec2D> = Vec::new();
    stack.push((x, y).into());

    loop {
        let Some(pos) = stack.pop() else {
            break;
        };
        if !unique && visited.contains(&pos) {
            continue;
        }
        visited.push(pos);
        let score = map.score(pos.x, pos.y);

        if score == 9 {
            total_score += 1;
            continue;
        }

        let neighbors = get_neighbors(pos.x, pos.y, map);

        for neighbor in neighbors {
            let neighbor_score = map.score(neighbor.x, neighbor.y);
            let diff = neighbor_score - score;

            if diff == 1 {
                stack.push(neighbor);
            }
        }
    }

    total_score
}

fn find_all_trailhead_scores(map: &Map, unique: bool) -> Vec<TrailHead> {
    let mut all_trail_heads: Vec<TrailHead> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map.score(x as i32, y as i32) != 0 {
                continue;
            }
            let score = dfs(x as i32, y as i32, map, unique);

            all_trail_heads.push(TrailHead {
                pos: (x as i32, y as i32).into(),
                score,
            });
        }
    }

    all_trail_heads
}

fn part01() {
    let input = get_input("input.txt");
    let map = parse_input(input);
    println!("{}", map);

    let trail_heads = find_all_trailhead_scores(&map, false);
    let sum = trail_heads
        .iter()
        .map(|th| {
            println!("{:?}", th);
            th.score
        })
        .sum::<i32>();
    println!("{}", sum);
}

fn part02() {
    let input = get_input("input.txt");
    let map = parse_input(input);
    println!("{}", map);

    let trail_heads = find_all_trailhead_scores(&map, true);
    let sum = trail_heads
        .iter()
        .map(|th| {
            println!("{:?}", th);
            th.score
        })
        .sum::<i32>();
    println!("{}", sum);
}

fn main() {
    part01();
    part02();
}

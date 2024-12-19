use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    path,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
    }
}

impl std::ops::Add for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: Self) -> Self::Output {
        IVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Blocked,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Blocked => write!(f, "#"),
        }
    }
}

struct Map {
    start: IVec2,
    end: IVec2,
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.map[0].len() as i32 && y >= 0 && y < self.map.len() as i32
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let pos = IVec2::new(x as i32, y as i32);
                match pos {
                    _ if pos == self.start => write!(f, "S")?,
                    _ if pos == self.end => write!(f, "E")?,
                    _ => write!(f, "{:?}", self.map[y][x])?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> Map {
    let mut start: IVec2 = IVec2::new(0, 0);
    let mut end: IVec2 = IVec2::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Blocked,
                    'S' => {
                        start = IVec2::new(x as i32, y as i32);
                        Tile::Empty
                    }
                    'E' => {
                        end = IVec2::new(x as i32, y as i32);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Map { map, start, end }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Clone, Copy)]
enum Direction {
    #[default]
    Right,
    Up,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Action {
    Move,
    TurnRight,
    TurnLeft,
}

impl Action {
    fn get_cost(&self) -> i32 {
        match self {
            Action::Move => 1,
            Action::TurnRight => 1000,
            Action::TurnLeft => 1000,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path {
    path: Vec<Action>,
    start_position: IVec2,
    start_direction: Direction,
    current_position: IVec2,
    current_direction: Direction,
}

impl Path {
    fn cost(&self) -> i32 {
        self.path.iter().map(Action::get_cost).sum()
    }
}

impl std::hash::Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current_position.hash(state);
    }
}

impl Path {
    fn new(start_position: IVec2, start_direction: Direction) -> Path {
        Path {
            path: vec![],
            start_position,
            start_direction,
            current_direction: start_direction,
            current_position: start_position,
        }
    }

    fn push(&mut self, action: Action) {
        self.path.push(action);
    }

    fn push_checked(&mut self, action: Action, map: &Map) {
        // make sure we don't go out of bounds
        // or hit a wall
        match action {
            Action::TurnRight => {
                self.current_direction = match self.current_direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };

                self.push(action);
            }
            Action::TurnLeft => {
                self.current_direction = match self.current_direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                };
                self.push(action);
            }
            Action::Move => {
                let pos = match self.current_direction {
                    Direction::Right => self.current_position + IVec2::new(1, 0),
                    Direction::Left => self.current_position + IVec2::new(-1, 0),
                    Direction::Up => self.current_position + IVec2::new(0, -1),
                    Direction::Down => self.current_position + IVec2::new(0, 1),
                };

                if map.map[pos.y as usize][pos.x as usize] == Tile::Blocked {
                    return;
                }
                if !map.within_bounds(pos.x, pos.y) {
                    return;
                }
                self.current_position = pos;
                self.push(action);
            }
        }
    }
}

fn draw_path(map: &Map, path: &Path) {
    let mut current_dir = path.start_direction;
    let visited = path
        .path
        .iter()
        .fold(vec![path.start_position], |mut acc, action| {
            match action {
                Action::Move => {
                    let current_pos = acc[acc.len() - 1];
                    let pos = match current_dir {
                        Direction::Right => current_pos + IVec2::new(1, 0),
                        Direction::Left => current_pos + IVec2::new(-1, 0),
                        Direction::Up => current_pos + IVec2::new(0, -1),
                        Direction::Down => current_pos + IVec2::new(0, 1),
                    };
                    acc.push(pos);
                }
                Action::TurnLeft => {
                    current_dir = match current_dir {
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Left,
                    };
                }
                Action::TurnRight => {
                    current_dir = match current_dir {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Right,
                    };
                }
            }
            acc
        });
    for y in 0..map.map.len() {
        for x in 0..map.map[0].len() {
            if visited.contains(&IVec2::new(x as i32, y as i32)) {
                print!("X");
            } else if map.map[y][x] == Tile::Blocked {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.cost();
        let other_cost = other.cost();

        self_cost.cmp(&other_cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_best_path(map: &Map) -> Path {
    let mut queue: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    let mut visited: HashSet<Path> = HashSet::new();

    for action in [Action::Move, Action::TurnRight, Action::TurnLeft] {
        let mut path = Path::new(map.start, Direction::default());
        path.push_checked(action, map);
        queue.push(Reverse(path));
    }

    while let Some(Reverse(path)) = queue.pop() {
        if !visited.insert(path.clone()) {
            continue;
        }

        let steps = path.path.iter().filter(|a| *a == &Action::Move).count();
        let turns = path.path.iter().filter(|a| *a != &Action::Move).count();
        // println!(
        //     "Checking a {:?} move at {:?}, looking {:?}, cost: {}, turns: {}, steps: {}",
        //     path.path[path.path.len() - 1],
        //     path.current_position,
        //     path.current_direction,
        //     path.cost(),
        //     turns,
        //     steps
        // );
        // draw_path(map, &path);

        let pos = path.current_position;

        if pos == map.end {
            return path.clone();
        }

        for action in [Action::Move, Action::TurnRight, Action::TurnLeft] {
            let prev_action = &path.path[path.path.len() - 1];
            if prev_action == &action {
                continue;
            }

            if path.path.len() >= 3 {
                let last_three = &path.path[path.path.len() - 3..path.path.len()];
                if last_three.len() == 3
                    && last_three
                        .iter()
                        .all(|action| *action == Action::TurnLeft || *action == Action::TurnRight)
                    && (action == Action::TurnLeft || action == Action::TurnRight)
                {
                    continue;
                }
            }
            let mut new_path = path.clone();
            new_path.push_checked(action, map);
            queue.push(Reverse(new_path));
        }
    }

    panic!("No path found")
}

fn part01() {
    let input = get_input("example.txt");
    let map = parse_input(&input);
    println!("{:?}", map);

    let path = find_best_path(&map);

    let cost = path.cost();

    println!("Cost: {}", cost);
}

fn part02() {}

fn main() {
    part01();
}

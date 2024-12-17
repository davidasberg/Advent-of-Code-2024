use core::fmt;
use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Object {
    Robot,
    Wall,
    Box,
    Empty,
    LeftBox,
    RightBox,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Robot => write!(f, "@"),
            Object::Wall => write!(f, "#"),
            Object::Box => write!(f, "O"),
            Object::Empty => write!(f, "."),
            Object::LeftBox => write!(f, "["),
            Object::RightBox => write!(f, "]"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
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

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_vec(&self) -> IVec2 {
        match self {
            Dir::Up => IVec2::new(0, -1),
            Dir::Down => IVec2::new(0, 1),
            Dir::Left => IVec2::new(-1, 0),
            Dir::Right => IVec2::new(1, 0),
        }
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dir::Up => write!(f, "^"),
            Dir::Down => write!(f, "v"),
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
        }
    }
}

struct Map {
    map: Vec<Vec<Object>>,
    robot_pos: IVec2,
}

impl Map {
    fn is_within_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0
            && pos.x < self.map[0].len() as i32
            && pos.y >= 0
            && pos.y < self.map.len() as i32
    }

    fn get(&self, pos: IVec2) -> Option<Object> {
        if self.is_within_bounds(pos) {
            Some(self.map[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn set(&mut self, pos: IVec2, object: Object) {
        assert!(self.is_within_bounds(pos));
        self.map[pos.y as usize][pos.x as usize] = object;
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let c = match self.map[y][x] {
                    Object::Robot => '@',
                    Object::Wall => '#',
                    Object::Box => '0',
                    Object::Empty => '.',
                    Object::LeftBox => '[',
                    Object::RightBox => ']',
                };
                write!(f, "{}", c)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str, scaled: bool) -> (Map, Vec<Dir>) {
    let (map, instructions) = input.trim().split_once("\n\n").unwrap();

    let objects: Vec<Vec<Object>> = map
        .lines()
        .map(|c| {
            c.chars()
                .map(|c| match c {
                    '#' => Object::Wall,
                    '.' => Object::Empty,
                    '@' => Object::Robot,
                    'O' => Object::Box,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let objects = if scaled {
        let mut new_objects = Vec::new();
        for y in 0..objects.len() {
            let mut row = Vec::new();
            for x in 0..objects[y].len() {
                match objects[y][x] {
                    Object::Box => row.extend_from_slice(&[Object::LeftBox, Object::RightBox]),
                    Object::Empty => row.extend_from_slice(&[Object::Empty, Object::Empty]),
                    Object::Wall => row.extend_from_slice(&[Object::Wall, Object::Wall]),
                    Object::Robot => row.extend_from_slice(&[Object::Robot, Object::Empty]),
                    _ => (),
                }
            }
            new_objects.push(row);
        }
        new_objects
    } else {
        objects
    };

    let mut robot_pos = IVec2::new(0, 0);
    for y in 0..objects.len() {
        for x in 0..objects[y].len() {
            if objects[y][x] == Object::Robot {
                robot_pos = IVec2::new(x as i32, y as i32);
                break;
            }
        }
    }

    let map = Map {
        map: objects,
        robot_pos,
    };

    let instructions = instructions
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '^' => Dir::Up,
            _ => unreachable!(),
        })
        .collect();

    (map, instructions)
}

fn move_cell(cell: IVec2, dir: Dir, map: &mut Map) -> bool {
    let new_pos = cell + dir.to_vec();

    if let (Some(cell_object), Some(new_object)) = (map.get(cell), map.get(new_pos)) {
        println!(
            "moving {:?} {:?} from {:?} to {:?}",
            dir, cell_object, cell, new_pos
        );
        match new_object {
            Object::Empty => {
                match cell_object {
                    Object::Robot => {
                        map.set(cell, Object::Empty);
                        map.set(new_pos, Object::Robot);
                        map.robot_pos = new_pos
                    }
                    Object::Box => {
                        map.set(new_pos, Object::Box);
                        map.set(cell, Object::Empty);
                    }
                    Object::LeftBox => {
                        map.set(new_pos, Object::LeftBox);
                        map.set(cell, Object::RightBox);
                    }
                    Object::RightBox => {
                        map.set(new_pos, Object::RightBox);
                        map.set(cell, Object::LeftBox);
                    }
                    _ => (),
                }
                return true;
            }
            Object::Box | Object::RightBox | Object::LeftBox => {
                if move_cell(new_pos, dir, map) {
                    map.set(cell, Object::Empty);
                    map.set(new_pos, cell_object);
                    if cell_object == Object::Robot {
                        map.robot_pos = new_pos;
                    }
                    return true;
                }
            }
            Object::Wall => (),
            Object::Robot => unreachable!(),
        }
    }
    false
}

fn part01() {
    let input = get_input("example.txt");

    let (mut map, instructions) = parse_input(&input, false);

    for instruction in instructions {
        move_cell(map.robot_pos, instruction, &mut map);
        println!("{:?}", map);
    }

    let mut boxes: Vec<IVec2> = Vec::new();
    for y in 0..map.map.len() {
        for x in 0..map.map[y].len() {
            if map.map[y][x] == Object::Box {
                boxes.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    let sum = boxes.iter().map(|IVec2 { x, y }| y * 100 + x).sum::<i32>();

    println!("{}", sum);
    // for y in 0..map.map.len() {
    //     for x in 0..map.map[y].len() {
    //         print!("{:?}", map.map[y][x]);
    //     }
    //     println!();
    // }

    // println!("{:?}", instructions);
}

fn part02() {
    let input = get_input("example.txt");

    let (mut map, instructions) = parse_input(&input, true);

    for instruction in instructions {
        move_cell(map.robot_pos, instruction, &mut map);
        println!("{:?}", map);
    }

    let mut boxes: Vec<IVec2> = Vec::new();
    for y in 0..map.map.len() {
        for x in 0..map.map[y].len() {
            if map.map[y][x] == Object::LeftBox {
                boxes.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    let sum = boxes.iter().map(|IVec2 { x, y }| y * 100 + x).sum::<i32>();

    println!("{}", sum);
    // for y in 0..map.map.len() {
    //     for x in 0..map.map[y].len() {
    //         print!("{:?}", map.map[y][x]);
    //     }
    //     println!();
    // }

    // println!("{:?}", instructions);
}

fn main() {
    part01();
    part02();
}

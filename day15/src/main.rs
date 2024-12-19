use core::fmt;
use std::{
    cell,
    ops::{Add, Sub},
};

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

    fn reverse(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
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

#[derive(Clone)]
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

    // fn can_move_cell(&self, cell: IVec2, dir: Dir) -> bool {
    //     let new_cell = cell + dir.to_vec();

    //     match self.get(cell) {
    //         Some(Object::Empty) => true,
    //         Some(Object::Box) | Some(Object::Robot) => self.can_move_cell(new_cell, dir),
    //         Some(Object::RightBox) => {
    //             self.can_move_cell(new_cell, dir)
    //                 && self.can_move_cell(new_cell + IVec2::new(-1, 0), dir)
    //         }
    //         Some(Object::LeftBox) => {
    //             self.can_move_cell(new_cell, dir)
    //                 && self.can_move_cell(new_cell + IVec2::new(1, 0), dir)
    //         }
    //         Some(Object::Wall) => false,
    //         None => false,
    //     }
    // }

    fn try_move_cell(&mut self, cell: IVec2, dir: Dir) -> bool {
        let new_cell = cell + dir.to_vec();
        let cell_obj = self.get(cell);
        // println!("trying to move {:?} {:?}", cell_obj, dir);

        let mut new_map = self.clone();

        let cell_obj = match cell_obj {
            Some(Object::Wall) | Some(Object::Empty) | None => return false,
            _ => cell_obj.unwrap(),
        };

        let can_move = match new_map.get(new_cell) {
            Some(Object::Empty) => true,
            Some(Object::Robot) => new_map.try_move_cell(new_cell, dir),
            Some(Object::Box) => new_map.try_move_cell(new_cell, dir),
            Some(Object::RightBox) => {
                new_map.try_move_cell(new_cell + IVec2::new(-1, 0), dir)
                    && new_map.try_move_cell(new_cell, dir)
            }
            Some(Object::LeftBox) => {
                new_map.try_move_cell(new_cell + IVec2::new(1, 0), dir)
                    && new_map.try_move_cell(new_cell, dir)
            }
            Some(Object::Wall) => false,
            None => false,
        };

        if can_move {
            new_map.set(cell, Object::Empty);
            new_map.set(new_cell, cell_obj);
            new_map.robot_pos = new_cell;
            *self = new_map;
            true
        } else {
            false
        }
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

fn part01() {
    let input = get_input("input.txt");

    let (mut map, instructions) = parse_input(&input, false);

    println!("{:?}", map);
    for instruction in instructions {
        map.try_move_cell(map.robot_pos, instruction);
        // println!("{:?}", map);
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
}

fn part02() {
    let input = get_input("input.txt");

    let (mut map, instructions) = parse_input(&input, true);

    println!("{:?}", map);
    for instruction in instructions {
        map.try_move_cell(map.robot_pos, instruction);
        // println!("{:?}", map);
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

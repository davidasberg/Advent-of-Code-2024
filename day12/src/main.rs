use core::fmt;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Deref,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PlantId(char);

impl Deref for PlantId {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        IVec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: Self) -> Self::Output {
        IVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<PlantId>>);

impl Deref for Map {
    type Target = Vec<Vec<PlantId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for p in row {
                write!(f, "{}", p.0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Region {
    plant_id: PlantId,
    locations: Vec<IVec2>,
}

fn get_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn parse_input(input: &str) -> Map {
    Map(input
        .lines()
        .map(|l| l.chars().map(PlantId).collect::<Vec<PlantId>>())
        .collect())
}

fn get_neighbors(cell: IVec2) -> Vec<IVec2> {
    vec![
        IVec2::new(cell.x - 1, cell.y),
        IVec2::new(cell.x + 1, cell.y),
        IVec2::new(cell.x, cell.y - 1),
        IVec2::new(cell.x, cell.y + 1),
    ]
}

fn get_neighbors_checked(cell: IVec2, map: &Map) -> Vec<IVec2> {
    let mut neighbors = Vec::new();
    if cell.x > 0 {
        neighbors.push(IVec2::new(cell.x - 1, cell.y));
    }
    if cell.x < map[0].len() as i32 - 1 {
        neighbors.push(IVec2::new(cell.x + 1, cell.y));
    }
    if cell.y > 0 {
        neighbors.push(IVec2::new(cell.x, cell.y - 1));
    }
    if cell.y < map.len() as i32 - 1 {
        neighbors.push(IVec2::new(cell.x, cell.y + 1));
    }
    neighbors
}

fn find_regions(map: &Map) -> Vec<Region> {
    let mut checked_cells: HashSet<IVec2> = HashSet::new();
    let mut regions = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if checked_cells.contains(&IVec2::new(x as i32, y as i32)) {
                continue;
            }

            checked_cells.insert(IVec2::new(x as i32, y as i32));

            let mut region = Region {
                plant_id: map[y][x],
                locations: Vec::new(),
            };

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();

            queue.push_back(IVec2::new(x as i32, y as i32));
            loop {
                if queue.is_empty() {
                    break;
                }

                let cell = queue.pop_front().unwrap();
                if visited.contains(&cell) {
                    continue;
                }
                visited.insert(cell);

                if map[cell.y as usize][cell.x as usize] != region.plant_id {
                    continue;
                }

                region.locations.push(cell);
                checked_cells.insert(cell);

                queue.extend(get_neighbors_checked(cell, map));
            }

            regions.push(region);
        }
    }

    regions
}

fn calc_perimeter(region: &Region, map: &Map) -> i32 {
    let mut perimeter = 0;
    for &cell in &region.locations {
        for neighbor in get_neighbors(cell) {
            // if out of bounds, its perimeter
            // or if the neighbor is not the same as the region
            if neighbor.y < 0
                || neighbor.y >= map.len() as i32
                || neighbor.x < 0
                || neighbor.x >= map[0].len() as i32
                || region.plant_id != map[neighbor.y as usize][neighbor.x as usize]
            {
                perimeter += 1;
            }
        }
    }
    perimeter
}

enum SideDir {
    Vertical,
    Horizontal,
}

impl From<IVec2> for SideDir {
    fn from(v: IVec2) -> Self {
        if v.x == 0 {
            SideDir::Horizontal
        } else {
            SideDir::Vertical
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Side {
    start: IVec2,
    end: IVec2,
}

impl Side {
    fn contains(&self, cell: IVec2) -> bool {
        self.start.x <= cell.x
            && cell.x <= self.end.x
            && self.start.y <= cell.y
            && cell.y <= self.end.y
    }

    fn add_cell(&mut self, cell: &IVec2) {
        if cell.x < self.start.x {
            self.start.x = cell.x;
        }
        if cell.x > self.end.x {
            self.end.x = cell.x;
        }
        if cell.y < self.start.y {
            self.start.y = cell.y;
        }
        if cell.y > self.end.y {
            self.end.y = cell.y;
        }
    }

    fn merge(&mut self, other: &Side) {
        self.start.x = min(self.start.x, other.start.x);
        self.end.x = max(self.end.x, other.end.x);
        self.start.y = min(self.start.y, other.start.y);
        self.end.y = max(self.end.y, other.end.y);
    }
}

fn count_sides(region: &Region, map: &Map) -> i32 {
    let mut sides: Vec<Side> = Vec::new();
    for &cell in &region.locations {
        for neighbor in get_neighbors(cell) {
            // if out of bounds, its perimeter
            // or if the neighbor is not the same as the region
            if neighbor.y < 0
                || neighbor.y >= map.len() as i32
                || neighbor.x < 0
                || neighbor.x >= map[0].len() as i32
                || region.plant_id != map[neighbor.y as usize][neighbor.x as usize]
            {
                let dir = (neighbor - cell).into();

                let dir = match dir {
                    SideDir::Vertical => IVec2::new(0, 1),
                    SideDir::Horizontal => IVec2::new(1, 0),
                };
                let mut sides_mut = sides.iter_mut();
                let above = sides_mut.find(|s| s.contains(neighbor - dir));

                let below = sides_mut.find(|s| s.contains(neighbor + dir));

                match (above, below) {
                    (Some(a), Some(b)) => {
                        a.merge(b);
                        sides.remove(sides.iter().position(|s| s == b).unwrap());
                    }
                    (Some(a), None) => {
                        a.add_cell(&(neighbor - dir));
                    }
                    (None, Some(b)) => {
                        b.add_cell(&(neighbor + dir));
                    }
                    (None, None) => {
                        sides.push(Side {
                            start: neighbor,
                            end: neighbor,
                        });
                    }
                }
            }
        }
    }

    sides.len() as i32
}

fn calc_area(region: &Region) -> i32 {
    region.locations.len() as i32
}

fn calc_price(region: &Region, map: &Map) -> i32 {
    let perimeter = calc_perimeter(region, map);
    let area = calc_area(region);
    perimeter * area
}

fn calc_discount_price(region: &Region, map: &Map) -> i32 {
    let sides = count_sides(region, map);
    let area = calc_area(region);
    println!(
        "A region of {} plants with price of {} * {} = {}",
        region.plant_id.0,
        area,
        sides,
        area * sides
    );
    sides * area
}

fn part01() {
    let input = get_input("input.txt");
    let map = parse_input(&input);

    let regions = find_regions(&map);

    let total_price = regions.iter().map(|r| calc_price(r, &map)).sum::<i32>();
    println!("total price {}", total_price);
}

fn part02() {
    let input = get_input("example.txt");
    let map = parse_input(&input);

    let regions = find_regions(&map);

    let total_price = regions
        .iter()
        .map(|r| calc_discount_price(r, &map))
        .sum::<i32>();

    println!("total price {}", total_price);
}

fn main() {
    part01();
    part02();
}

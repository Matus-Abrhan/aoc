use std::fs; 
use std::collections::{HashMap, HashSet};
use std::hash::{Hasher, Hash};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            return string;
        },
        Err(e) => {
            println!("read error, {:?}", e);
            return String::new();
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn get(&self, direction: Direction) -> Point {
        match direction {
            Direction::North => { return Point{x: self.x , y: self.y-1} },
            Direction::South => { return Point{x: self.x , y: self.y+1} },
            Direction::East => { return Point{x: self.x+1, y: self.y} },
            Direction::West => { return Point{x: self.x-1, y: self.y} },
        };
    }
}
impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
fn main() {
    let data = read_file("input");

    let mut start: Point = Point { x: 0, y: 0};
    let mut pipes: HashMap<Point, (Point, Point)> = HashMap::new();
    let mut uncolored: HashMap<Point, char> = HashMap::new();
    for (l_idx, line) in data.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            let point = Point{x: c_idx as i32, y: l_idx as i32};
            uncolored.insert(point.clone(), c);
            match c {
                '|' => { 
                    pipes.insert(point.clone(), (point.get(Direction::North), point.get(Direction::South)));
                },       
                '-' => {
                    pipes.insert(point.clone(), (point.get(Direction::West), point.get(Direction::East)));
                },
                'L' => {
                    pipes.insert(point.clone(), (point.get(Direction::North), point.get(Direction::East)));
                },
                'J' => {
                    pipes.insert(point.clone(), (point.get(Direction::North), point.get(Direction::West)));
                },
                '7' => {
                    pipes.insert(point.clone(), (point.get(Direction::South), point.get(Direction::West)));
                },
                'F' => {
                    pipes.insert(point.clone(), (point.get(Direction::South), point.get(Direction::East)));
                },
                'S' => { 
                    start = point.clone();  
                },
                _ => {}
            };
            
        }
    }
    let mut current = Point { x: 0, y: 0};
    for (point, (v1, _v2)) in pipes.iter() {
        if *v1 == start {
            current = point.clone();
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start.clone());
    visited.insert(current.clone());
    let mut res: usize = 0;
    loop {
        res += 1;
        let (p1, p2) = pipes.get(&current).unwrap();

        if !visited.contains(p1) {
            visited.insert(p1.clone());
            current = *p1;
        } else if !visited.contains(p2){
            visited.insert(p2.clone());
            current = *p2;
        } else {
            break;
        }
    }
    println!("{:}", (res+1)/2);

    res = 0;
    for point in uncolored.keys().filter(|p| !visited.contains(p)) {
        let mut counter = 0;
        let mut ray = point.clone();
        let mut l_flag = false;
        let mut r_flag = false;
        while ray.y >= 0 {
            if visited.contains(&ray) {
                match *uncolored.get(&ray).unwrap() {
                    '-' => { counter += 1 },
                    'L'|
                    'F' => {
                        if l_flag {
                            l_flag = false;
                            counter += 1;
                        } else if r_flag {
                            r_flag = false;
                        }else {
                            r_flag = true;
                        }
                    },
                    'J'|
                    '7' => {
                        if r_flag {
                            r_flag = false;
                            counter += 1;
                        } else if l_flag {
                            l_flag = false;
                        } else {
                            l_flag = true;
                        }
                    },
                    'S' => {
                        l_flag = false;
                        r_flag = false;
                        counter += 1;
                    },
                    _ => {}
                };
            }
            ray.y -= 1;
        }
        if counter%2 == 1 {
            res += 1;
        }
    }
    println!("{:?}", res);

}

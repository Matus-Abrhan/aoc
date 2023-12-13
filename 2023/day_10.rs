use std::{fs, collections::HashMap};

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

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn north(self: &Point) -> Point {
        let point = Point { x: self.x, y: self.y-1 };
        return point;
    }
    fn south(self: &Point) -> Point {
        let point = Point { x: self.x, y: self.y+1 };
        return point;
    }
    fn west(self: &Point) -> Point {
        let point =  Point { x: self.x-1, y: self.y };
        return point;
    }
    fn east(self: &Point) -> Point {
        let point = Point { x: self.x+1, y: self.y };
        return point;
    }
    fn copy(self: &Point) -> Point {
        let point = Point { x: self.x, y: self.y };
        return point;
    }
}

fn main() {
    let data = read_file("input");

    let mut start: Point = Point{x:0, y:0};
    let mut pipes: HashMap<Point, (Point, Point)> = HashMap::new();
    for (line_i, line) in data.lines().enumerate() {
        for (column_i, c) in line.chars().enumerate() {
            let point: Point = Point{x: column_i as i32, y: line_i as i32};
            match c {
                '|' => _ = pipes.insert(point.copy(), (point.north(), point.south())),
                '-' => _ = pipes.insert(point.copy(), (point.east(), point.west())),
                'L' => _ = pipes.insert(point.copy(), (point.north(), point.east())),
                'J' => _ = pipes.insert(point.copy(), (point.north(), point.west())),
                '7' => _ = pipes.insert(point.copy(), (point.south(), point.west())),
                'F' => _ = pipes.insert(point.copy(), (point.south(), point.east())),
                'S' => start = Point{x: column_i as i32, y: line_i as i32},
                _ => {}
            };
            
        }
    }
    println!("{:?}", pipes);

    let mut current: Point = Point{x:0, y:0};
    for (k, (v1, v2)) in &pipes {
        if *v1 == start || *v2 == start {
            current = k.copy();
            break;
        }
    }

    println!("{:?}", current);
    let mut res: usize = 0;
    let mut previous: Point = start.copy();
    let mut next: Point = Point { x: 0, y: 0 };
    while current != start {

        let (p1, p2) = pipes.get(&current).unwrap();

        if *p1 == previous {
            next = p2.copy();
        }else {
            next = p1.copy();
        }
        previous = current.copy();
        current = next.copy();
        res += 1;

    }
    println!("{:}", (res+1)/2);



    
}

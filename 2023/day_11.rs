use std::{fs, collections::HashSet, i128};


fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            return string
        },
        Err(e) => {
            println!("{:}", e);
            return String::new();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i128,
    y: i128
}

fn main() {
    let data = read_file("input");

    let mut galaxy_columns: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<Point> = Vec::new();
    let mut line_exp = 0;
    for (l_idx, line) in data.lines().enumerate() {
        let mut count = 0;
        for (c_idx, c) in line.chars().enumerate() {
            if c == '#' {
                count += 1;
                galaxies.push(Point{x: c_idx as i128, y: (l_idx+line_exp) as i128});
                galaxy_columns.insert(c_idx);
            }
        }
        if count == 0 {
            line_exp += 999999;
        }
    }

    let mut galaxies_exp: Vec<Point> = Vec::new();
    for galaxy in galaxies.iter() {
        let mut column_exp = 0;
        for i in 0..galaxy.x {
            if !galaxy_columns.contains(&(i as usize )){
                column_exp += 999999;
            }
        }
        galaxies_exp.push(Point{x: galaxy.x+(column_exp) as i128, y: galaxy.y});
    }

    let mut sum: i128 = 0;
    for (a_idx, a) in galaxies_exp.iter().enumerate() {
        for b in  galaxies_exp.iter().rev().take(galaxies_exp.len()-a_idx) {
            sum += (a.x-b.x).abs() + (a.y-b.y).abs();
        }
    }
    println!("{:?}", sum);
}

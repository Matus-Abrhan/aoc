use std::{fs, collections::HashMap, usize};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            return string;
        }
        Err(e) => {
            println!("{:?}", e);
            return String::new();
        }
    }
}

#[derive(Debug, PartialEq)]
enum RockType {
    Round,
    Cube
}

#[derive(Debug)]
struct Rock {
    x: i32,
    y: i32,
    rock_type: RockType
}

fn north(rocks: &mut Vec<Rock>) {
    rocks.sort_by(|a, b| a.y.cmp(&b.y));

    let mut rock_map: HashMap<i32, i32> = HashMap::new();
    for rock in rocks {
        match rock.rock_type {
            RockType::Cube => {
                rock_map.entry(rock.x)
                        .and_modify(|v| { *v = rock.y})
                        .or_insert(rock.y);
            },
            RockType::Round => {
                let current_height = *rock_map.entry(rock.x).or_insert(-1);
                rock.y = current_height + 1;
                rock_map.entry(rock.x).and_modify(|v| { *v = rock.y });
            }
        };
    }
}

fn west(rocks: &mut Vec<Rock>) {
    rocks.sort_by(|a, b| a.x.cmp(&b.x));

    let mut rock_map: HashMap<i32, i32> = HashMap::new();
    for rock in rocks {
        match rock.rock_type {
            RockType::Cube => {
                rock_map.entry(rock.y)
                        .and_modify(|v| { *v = rock.x})
                        .or_insert(rock.x);
            },
            RockType::Round => {
                let current_height = *rock_map.entry(rock.y).or_insert(-1);
                rock.x = current_height + 1;
                rock_map.entry(rock.y).and_modify(|v| { *v = rock.x });
            }
        };
    }
}

fn south(rocks: &mut Vec<Rock>, num_lines: i32) {
    rocks.sort_by(|a, b| a.y.cmp(&b.y).reverse());

    let mut rock_map: HashMap<i32, i32> = HashMap::new();
    for rock in rocks {
        match rock.rock_type {
            RockType::Cube => {
                rock_map.entry(rock.x)
                        .and_modify(|v| { *v = rock.y})
                        .or_insert(rock.y);
            },
            RockType::Round => {
                let current_height = *rock_map.entry(rock.x).or_insert(num_lines);
                rock.y = current_height - 1;
                rock_map.entry(rock.x).and_modify(|v| { *v = rock.y });
            }
        };
    }
}

fn east(rocks: &mut Vec<Rock>, num_columns: i32) {
    rocks.sort_by(|a, b| a.x.cmp(&b.x).reverse());

    let mut rock_map: HashMap<i32, i32> = HashMap::new();
    for rock in rocks {
        match rock.rock_type {
            RockType::Cube => {
                rock_map.entry(rock.y)
                        .and_modify(|v| { *v = rock.x})
                        .or_insert(rock.x);
            },
            RockType::Round => {
                let current_height = *rock_map.entry(rock.y).or_insert(num_columns);
                rock.x = current_height - 1;
                rock_map.entry(rock.y).and_modify(|v| { *v = rock.x });
            }
        };
    }
}


fn main() {
    let data = read_file("test_input");
    let num_lines: i32 = data.lines().count() as i32;
    let num_columns: i32 = data.split_once('\n').unwrap().0.len() as i32;
    let mut rocks: Vec<Rock> = Vec::new();

    for (line_idx, line) in data.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rocks.push(Rock{x: c_idx as i32, y: line_idx as i32, rock_type: RockType::Cube});
                },
                'O' => {
                    rocks.push(Rock { x: c_idx as i32, y: line_idx as i32, rock_type: RockType::Round})
                }
                _ => {}
            }
        }
    }

    for _ in 0..1000000000 {
        north(&mut rocks);
        west(&mut rocks);
        south(&mut rocks, num_lines);
        east(&mut rocks, num_columns);
    }
    
    let mut sum = 0;
    for rock in rocks.iter().filter(|r| r.rock_type == RockType::Round){
        sum += (num_lines as i32) - rock.y;
    }
    println!("{:?}", sum);

}

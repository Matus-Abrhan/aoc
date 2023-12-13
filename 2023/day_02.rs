use std::{fs, collections::HashMap};
use std::hash::{Hash, Hasher};

static RED: i32 = 12;
static GREEN: i32 = 13;
static BLUE: i32 = 14;

enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn from(input: &str) -> Option<Color> {
        match input {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None
        }
    }
}

impl Hash for Color{
    fn hash<H: Hasher>(&self, state: &mut H)  {
        match self {
            Color::Red => 1.hash(state),
            Color::Green => 2.hash(state), 
            Color::Blue => 3.hash(state)
        };
    }
}
impl PartialEq for Color{
    fn eq(&self, other: &Self) -> bool {

        return match (self, other) {
            (&Color::Red, &Color::Red) => true,
            (&Color::Green, &Color::Green) => true,
            (&Color::Blue, &Color::Blue) => true,
            _ => false
        };
    }
}
impl Eq for Color {}

fn read_file(path: &str) -> String {
    let data = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => { println!("could not read, exception {:?}", e); String::new() }
    };

    return data;
}

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let lines = read_file("input");
    for line in lines.lines() {
       let mut bag: HashMap<Color, i32> = HashMap::new();
       let id = line.split(':').collect::<Vec<&str>>()[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
       for round in line.split(':').collect::<Vec<&str>>()[1].trim().split(';') {
            for item in round.split(',') {
                let (count, color) = item.trim().split_once(' ').unwrap();
                //println!("{:?},{:?}", count, color);
                let mut count = count.parse::<i32>().unwrap();

                match Color::from(&color) {
                    Some(c) => bag.entry(c).and_modify(|v| if v < &mut count { *v = count } ).or_insert(count),
                    None => panic!("could not recognise color")
                };
                    
                
            }
       }
       if RED >= *bag.get(&Color::Red).unwrap() && GREEN >= *bag.get(&Color::Green).unwrap() && BLUE >= *bag.get(&Color::Blue).unwrap() {
            sum1 += id;
       }
       sum2 += *bag.get(&Color::Red).unwrap() * *bag.get(&Color::Green).unwrap() * *bag.get(&Color::Blue).unwrap();
       
    }
    println!("{:?}\n{:?}", sum1, sum2);
}

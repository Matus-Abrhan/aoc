use std::{fs, iter::zip}; 

fn read_file(path: &str) -> String{
    match fs::read_to_string(path) {
        Ok(string) => return string,
        Err(e) => {
            println!("Unsuccessful read, {:?}", e);
            return String::new();
        }
    };
}

fn main() {
    let data = read_file("input2");
    let mut lines = data.lines();

    let times_string: Vec<&str> = lines.next().unwrap().split_once(':').unwrap().1.trim().split(' ').filter(|s| !s.is_empty()).collect();
    let distances_string: Vec<&str> = lines.next().unwrap().split_once(':').unwrap().1.trim().split(' ').filter(|s| !s.is_empty()).collect();
    let times: Vec<usize> = times_string.iter().map(|t| t.parse().unwrap()).collect();
    let distances: Vec<usize> = distances_string.iter().map(|t| t.parse().unwrap()).collect();
    
    let mut res = 1;
    for (time, distance) in zip(times, distances){
        //println!("{:}, {:}", time, distance);
        let mut sum = 0;
        for i in 1..time-1 {
            let remaining_time = time-i;
            let d = remaining_time * i;
            if d > distance {
                sum += 1;
            }
        }
        println!("{:}", sum);
        res *= sum;
    }
    println!("{:}", res);
}

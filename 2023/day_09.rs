use std::fs;

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            return string;
        },
        Err(e) => {
            println!("Read failed, {:?}", e);
            return String::new();
        }
    };
}

fn predict(numbers: Vec<i32>) -> i32 {
    let mut new_numbers = Vec::new();

    for (i, n) in numbers.iter().skip(1).enumerate() {
        new_numbers.push(n-numbers.get(i).unwrap());
    }
    match new_numbers.iter().all(|n| *n == 0) {
        true => {
            return 0;
        },

        false => {
            let res = predict(new_numbers.clone());
            return new_numbers.first().unwrap() - res;
        }
    };
}

fn main() {
    let data = read_file("test_input");

    let mut sum = 0;
    for line in data.lines() {
        let numbers_data: Vec<&str> = line.split(' ').collect();
        let mut numbers: Vec<i32> = Vec::new();

        for n in numbers_data.iter().map(|n| n.parse::<i32>().unwrap()) {
            numbers.push(n);            

        }

        let res = numbers.first().unwrap() - predict(numbers.clone());

        sum += res;
    }
    println!("{:}", sum);
}

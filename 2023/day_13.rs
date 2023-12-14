use std::fs;

fn read_file(path: &str) -> String{
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

fn get_vertical(pattern: &String) -> i32 {
    let line_len = pattern.split_once('\n').unwrap().0.len();

    for i in 1..line_len {

        let mut flag = true;
        let mut smudge = false;
        for line in pattern.split_whitespace() {
            let mut iter1 = line.chars().rev().skip(line_len-i);
            let mut iter2 = line.chars().skip(i);
            loop {
                let elem1 = iter1.next();
                let elem2 = iter2.next();

                if elem1.is_none() || elem2.is_none() {
                    break;
                }else if elem1 != elem2 {
                    if smudge {
                        flag = false;
                        break;
                    }

                    smudge = true;
                }
            }
        }
        if flag && smudge {
            println!("{:?}", i);
            return i as i32;
        }
    }
    return 0;
}

fn get_horizontal(pattern: &String) -> i32 {
    let lines: Vec<&str> = pattern.split_whitespace().collect();
    
    for i in 1..lines.len() {
        let mut flag  = true;
        let mut smudge = false;
        let mut iter1 = lines.iter().rev().skip(lines.len()-i);
        let mut iter2 = lines.iter().skip(i);

        loop {
            let elem1 = iter1.next();
            let elem2 = iter2.next();

            if elem1.is_none() || elem2.is_none() {
                break;
            } 
            let elements_zipped = elem1.unwrap().chars().zip(elem2.unwrap().chars());
            for (e1, e2) in elements_zipped {
                if e1 != e2 {
                    if smudge {
                        flag = false;
                        break;
                    }
                    smudge = true;
                }
            }
        }
        if flag && smudge {
            println!("{:?}", i);
            return i as i32;
        }
    }
    return 0;
}

fn main() {
    let data = read_file("input");
    
    let mut sum_h = 0;
    let mut sum_v = 0;
    for pattern in data.split("\n\n") {
        sum_v += get_vertical(&pattern.to_string());
        sum_h += get_horizontal(&pattern.to_string());
    }
    
    println!("{:}", (100*sum_h) + sum_v);
}

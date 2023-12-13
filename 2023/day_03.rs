use std::{fs, collections::HashSet};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Part {
    number: String,
    line: usize,
    start_column: usize,
    length: usize, 
}

fn read_file(path: &str) -> String {
    let data = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => { println!("Read not successful, error {:?}", e); String::new()}
    };
    return data;
}

fn parse_line(line_string: &str, mut parts: Vec<Part>, mut symbols: Vec<Part>, line_num: usize) -> (Vec<Part>, Vec<Part>){
    //let mut line_string = String::from(line);

    let part = line_string.split('.').find(|s| !s.is_empty());
    
    match part {
        Some(part) => {
            if part.chars().all(|c| c.is_digit(10)) {
                parts.push(Part{
                    number: part.to_string(),
                    line: line_num+1,
                    start_column: line_string.find(part).unwrap()+1,
                    length: part.to_string().len()});

                let line_modified = line_string.replacen(part, &".".repeat(part.to_string().len()), 1);
                parse_line(&line_modified, parts, symbols, line_num)
            } else {   
                //println!(" Not number");
                let part = part.chars().find(|s| !s.is_digit(10)).unwrap();
                symbols.push(Part{
                    number: part.to_string(),
                    line: line_num +1,
                    start_column: line_string.find(part).unwrap()+1,
                    length: part.to_string().len()});
                let line_modified = line_string.replacen(part, &".".repeat(part.to_string().len()), 1);
                parse_line(&line_modified, parts, symbols, line_num)
            }
        },
        None => return (parts, symbols)//return found_parts
    }
}

fn main() {
    let data = read_file("input");

    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Part> = Vec::new();
    for (line_index, line) in data.lines().enumerate() {
       (parts, symbols) = parse_line(line, parts, symbols, line_index); 
    }

    let mut parts_set: HashSet<&Part> = HashSet::new();
    for symbol in &symbols {
        //let line_min = if symbol.line == 0 { 0 } else {symbol.line-1};
        let found = parts.iter().filter(|p| (symbol.line-1 <= p.line && symbol.line+1 >= p.line) && 
                                            (p.start_column-1 <= symbol.start_column && p.start_column+p.length >= symbol.start_column));
        //println!("{:?}", symbol.number);
        for f in found {
            //println!("{:?}", f.number);
            //sum += f.number.parse::<usize>().unwrap();
            parts_set.insert(f);
        }
    }

    let mut sum: usize = 0;
    for part in parts_set {
        sum += part.number.parse::<usize>().unwrap();
    }

    println!("{:?}", sum);

    let gears = symbols.iter().filter(|s| s.number == "*");
    let mut sum2 = 0;
    for symbol in gears {
        
        let found: Vec<&Part> = parts.iter().filter(|p| (symbol.line-1 <= p.line && symbol.line+1 >= p.line) && 
                                            (p.start_column-1 <= symbol.start_column && p.start_column+p.length >= symbol.start_column)).collect();
        
        if found.len() == 2 {
            sum2 +=  found.get(0).unwrap().number.parse::<usize>().unwrap() * found.get(1).unwrap().number.parse::<usize>().unwrap();
        }
    }
    println!("{:?}", sum2);
}

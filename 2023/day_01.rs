use std::fs;

fn read_file(path: &str) -> String {
    let data: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            print!("read not successful, exception {:?}", e);
            String::new()
        }
    };
    
    return data;
}

fn main() {
    let lines = read_file("input");
    let mut sum = 0;
    let nums_spelled = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in lines.lines(){
        let mut line_string: String = String::from(line);
        let mut num: usize = usize::MAX;
        loop{
            let mut min: usize = usize::MAX;
            for (idx, num_spelled) in nums_spelled.iter().enumerate() {
                match line_string.find(num_spelled) {
                    Some(x) => {
                        if x < min {
                            min = x;
                            num = idx;
                        }
                    }
                    None => continue
                }
            }
            if min == usize::MAX {
                break;
            }
            line_string = line_string.replacen(nums_spelled.get(num).unwrap(), (num+1).to_string().as_str(), 1);
        }
        //println!("{}", line_string);
        let nums: Vec<char> = line_string.chars().filter(|c| c.is_numeric()).collect();
        //print!("{:}{:}\n", nums.first().unwrap(), nums.last().unwrap());
        let mut s: String = String::new();
        s.push(*nums.first().unwrap());
        s.push(*nums.last().unwrap());

        sum += s.parse::<i32>().unwrap();
        //print!("{:}\n", s);
    }
    print!("{:}",sum);
}

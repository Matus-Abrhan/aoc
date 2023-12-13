use std::fs;

#[derive(Debug)]
struct Mapping {
    dest_range_start: Vec<usize>,
    source_range_start: Vec<usize>,
    range_length: Vec<usize>,
}

impl Mapping {
    fn map(&self, source_n: usize) -> usize {
        for (idx, source_start) in self.source_range_start.iter().enumerate() {
            let source_end = source_start+self.range_length.get(idx).unwrap()-1;

            if (source_start..=&source_end).contains(&&source_n) {
                let dest_n = self.dest_range_start.get(idx).unwrap() + (source_n - source_start); 
                return dest_n;
            }
        }
        return source_n;
    }

    fn map_range(&self, (start, len): (usize, usize)) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)> = Vec::new();
        let mut unmatched: Option<(usize, usize)> = None;
        let mut start_c = start.clone();
        let end_c = start + len - 1;
        let mut flag = true;

        while start_c <  end_c {
            for (idx, source_start) in self.source_range_start.iter().enumerate() {
                let source_end = source_start+self.range_length.get(idx).unwrap() - 1;
                if (source_start..=&source_end).contains(&&start_c) {
                    flag = false;
                    if unmatched.is_some(){
                        output.push(unmatched.unwrap().clone());
                        unmatched = None;
                    }

                    let dest_start = self.dest_range_start.get(idx).unwrap() + (start_c - source_start);  
                    if end_c > source_end {
                        output.push((dest_start.clone(), source_end - start_c)); //source_end - start_c
                        start_c = source_end+1;
                    }
                    else {
                        output.push((dest_start.clone(), end_c - start_c));
                        start_c = end_c;
                    }
                }
            }
            if flag {
                if unmatched.is_some() {
                    unmatched = Some((unmatched.unwrap().0, unmatched.unwrap().1 + 1));
                }
                else{
                    unmatched = Some((start_c, 0))
                } 
                start_c += 1;
            }
            flag = true;
        }
        if flag {
            if unmatched.is_some() {
                output.push(unmatched.unwrap().clone());
            }
        }
        return output;
    }
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            string
        }
        Err(e) => {
            println!(" Error reading file, {:?}", e);
            String::new()
        }
    }
}

fn main() {
    let data = read_file("input");
    
    let (seeds_line, lines) =  data.split_once("\n\n").unwrap();
    let mut seeds: Vec<usize> = Vec::new();
    for seed in seeds_line.split_once(':').unwrap().1.trim().split(' ') {
         seeds.push(seed.parse::<usize>().unwrap());
    } 

    let mut seeds_iter = seeds.iter();
    let mut seeds_ranges: Vec<(usize, usize)> = Vec::new();
    for _ in 0..seeds.len()/2 {
        seeds_ranges.push((*seeds_iter.next().unwrap(), *seeds_iter.next().unwrap()))
    }
    println!("{:?}", seeds);
    println!("{:?}", seeds_ranges);

    //let mut flag = true;
    //let mut seeds: Vec<usize> = Vec::new();
    for line in lines.split("\n\n") {
        let mut mapping = Mapping{dest_range_start: Vec::new(),
                                    source_range_start: Vec::new(),
                                    range_length: Vec::new()};
        for entry in line.split_once('\n').unwrap().1.split('\n').filter(|e| !e.is_empty()){
            let mut entry_iter = entry.splitn(3, ' ');
            mapping.dest_range_start.push(entry_iter.next().unwrap().parse::<usize>().unwrap());
            mapping.source_range_start.push(entry_iter.next().unwrap().parse::<usize>().unwrap());
            mapping.range_length.push(entry_iter.next().unwrap().parse::<usize>().unwrap());
        }
        //let mut new_seeds: Vec<usize>= Vec::new();
        //for seed in seeds {
        //    new_seeds.push(mapping.map(seed));
        //}
        //seeds = new_seeds;

        let mut new_seeds: Vec<(usize, usize)> = Vec::new();
        //if flag {
        for seed in seeds_ranges {
            new_seeds.append(&mut mapping.map_range(seed));
        }
            //flag = false;
        //}
        //else {
        //    for seed in seeds {
        //        new_seeds.push(mapping.map(seed));
        //    }
        //}
        seeds_ranges = new_seeds;
        println!("{:?}", seeds_ranges);
    }

    println!("{:?}", seeds_ranges.iter().min());
}

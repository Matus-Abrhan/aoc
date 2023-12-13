use std::{fs, collections::{HashMap, HashSet}};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(string) => {
            return string;
        },
        Err(e) => {
            println!("Read not succesful, {:?}", e);
            return String::new();
        }
    };
}

fn main() {
    let data = read_file("input");
    let mut data_iter = data.lines();
    let path = data_iter.next().unwrap();
    _ = data_iter.next();
    
    let mut nodes: HashSet<&str> = HashSet::new();
    let mut tree: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in data_iter {
        let (node, dest) = line.split_once(" = ").unwrap();
        let (l, r) = dest.trim_matches(|c| c=='(' || c==')').split_once(", ").unwrap();
        tree.insert(node, (l, r));
        if node.ends_with('A') {
            nodes.insert(node);
        }
    }
    // println!("{:?}", nodes);
    // println!("{:?}", path);
    let mut sum: usize = 1;

    for mut node in nodes {
        let mut steps: usize = 0;
        while !node.ends_with('Z'){
            for step in path.chars() {
                match step {
                    'L' => {
                        node = tree.get(node).unwrap().0;
                    }
                    'R' => {
                        node = tree.get(node).unwrap().1;
                    }
                    _ => panic!("wtf")
                };
                steps += 1;
            }
        }
        sum *= steps/path.len();
    }
    println!("{:}", sum*path.len());
}

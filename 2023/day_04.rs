use std::{fs, collections::{HashSet, HashMap}};

#[derive(Debug)]
struct Card {
    id: i32,
    win:  HashSet<String>,
    have: HashSet<String>,
}

fn read_fiile(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => {
            data
        }
        Err(e) => {
            println!("unsuccessful read, {:?}", e);
            String::new()
        }
    }
}

fn main() {
    let data = read_fiile("input");
    let mut sum = 0;
    let mut copies: HashMap<i32, i32> = HashMap::new();

    for line in data.lines() {
        let (card_id, rest) = line.split_once(':').unwrap();
        let (win_nums, has_nums) = rest.split_once('|').unwrap();
        
        let mut card = Card{id: 0, win: HashSet::new(),have: HashSet::new()};
        card.id = card_id.split_once(' ').unwrap().1.trim().parse::<i32>().unwrap();
        for n in win_nums.split(' ').filter(|n| !n.is_empty()) {
            card.win.insert(n.trim().to_string());
        }
        for n in has_nums.split(' ') {
            card.have.insert(n.trim().to_string());
        }

        let count: HashSet<&String> = card.win.intersection(&card.have).collect();    
        if count.len() > 0 {
            sum += 2_i32.pow((count.len()-1)as u32);
        }

        let multilpy: i32 = *copies.entry(card.id).or_insert(1);
        for i in 1..count.len()+1 {
            copies.entry(card.id+(i)as i32).and_modify(|v| *v+=multilpy).or_insert(1+multilpy);
        }
    }
    println!("{:?}", sum);

    println!("{:?}", copies.values().sum::<i32>());
}

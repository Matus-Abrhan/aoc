use std::{fs, collections::HashMap, cmp::Ordering};

struct Hand {
    cards: Vec<u32>,
    strength: u32,
    bid: usize
}


fn read_file(path: &str) -> String{
    match fs::read_to_string(path){
        Ok(string) => {
            return string
        },
        Err(e) => {
            println!("Could not read, error: {:?}", e);
            return String::new();
        }
    };
}

fn get_type(hand: &HashMap<u32, u32>) -> u32{
    let mut jokers = match hand.get(&1) {
        Some(v) => *v,
        None => 0
    };
    
    let mut hand = hand.clone();
    if jokers < 5 {
        _ = hand.remove(&1);
    }
    let mut h: Vec<_> = hand.values().cloned().collect(); h.sort(); h.reverse();
    let mut hand_iter = h.iter();
    let first = *hand_iter.next().unwrap();
    
    if 5 <= first + jokers {
        return 7;
    }else if 4 <= first+jokers {
        return 6;
    } else if 3 <= first+jokers {
        if first < 3 {
            jokers -= 3-first
        }
        let second = *hand_iter.next().unwrap();
        if 2 <= second+jokers {
            return 5;
        }
        else {
            return 4;
        }
    } else if 2 <= first+jokers{
        if first < 2 {
            jokers -= 2-first;
        }
        let second = *hand_iter.next().unwrap();
        if 2 <= second+jokers {
            return 3;
        }
        else {
            return 2;
        }
    } else {
        return 1;
    }
}

fn main() {
    let data = read_file("input");
    let mut hands: Vec<Hand> = Vec::new();
    for line in data.lines() {
        let mut cards: HashMap<u32, u32> = HashMap::new();
        let (hand_data, bid_data) = line.trim().split_once(' ').unwrap();
        println!("Hand: {:}, Bid: {:}", hand_data, bid_data);

        let mut card_values: Vec<u32> = Vec::new();
        for card in hand_data.chars() {
            let card_value = match card {
                'J' => 1,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                c => c.to_digit(10).unwrap()
            };
            card_values.push(card_value);
            cards.entry(card_value).and_modify(|v| *v+=1).or_insert(1);
        }
        let strength = get_type(&cards);
        let hand = Hand{cards: card_values,
                        strength: strength,
                        bid: bid_data.parse::<usize>().unwrap()};
        hands.push(hand);
    }
    hands.sort_by(|a, b| {
                    match a.strength.cmp(&b.strength) {
                        Ordering::Equal => {
                            for (x, y) in a.cards.iter().zip(&b.cards){
                                if x > y {
                                    return Ordering::Greater;
                                } else if x < y {
                                    return Ordering::Less;
                                }
                            }
                            return  Ordering::Equal;
                        },
                        ord => return ord
                    };
                });  
    let mut sum = 0;
    for (idx, hand) in hands.iter().enumerate() {
        println!("{:?}", hand.bid);
        sum += (idx+1)*hand.bid;
    }
    println!("{:?}", sum);

}

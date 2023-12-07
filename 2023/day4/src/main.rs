use std::{fs, collections::HashMap, vec};

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let mut memory = HashMap::new();
    let mut p1 = 0;
    let mut oricard: Vec<u32> = vec![];
    for line in file.lines() {
        let (card_id, other_line) = line.split_once(':').unwrap();
        let id: u32 = card_id.strip_prefix("Card ").unwrap().trim().parse::<u32>().unwrap();
        let (winning, mycard) = other_line.split_once('|').unwrap();
        let nb: Vec<u32> = winning.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let my_nb: Vec<u32> = mycard.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let mut card_point: u32 = 0;
        oricard.push(id);
        for leditnombre in my_nb {
            if let Some(_x) = nb.iter().position(|&x| x == leditnombre) {
                card_point += 1;
            }
        }
        for n in id+1..=id+card_point {
            if let Some(&mult) = memory.get(&id) {
                *memory.entry(n).or_insert(0) += mult + 1;
            } else {
                memory.insert(n, 1);
            }
        };

        if card_point > 1 {
            card_point = 2_u32.pow(card_point - 1); 
        }
        
        p1 += card_point;
    }
    println!("p1 : {p1}");
    println!("p2 : {:?}", memory.values().sum::<usize>() + oricard.len());

}

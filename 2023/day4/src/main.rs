use std::{fs, collections::HashMap};

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let mut memory = HashMap::new();
    let mut _p1 = 0;
    for line in file.lines() {
        let (card_id, other_line) = line.split_once(':').unwrap();
        let id: u32 = card_id.strip_prefix("Card ").unwrap().trim().parse::<u32>().unwrap();
        let (winning, mycard) = other_line.split_once('|').unwrap();
        let nb: Vec<u32> = winning.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let my_nb: Vec<u32> = mycard.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let mut card_point: u32 = 0;
        
        for leditnombre in my_nb {
            if let Some(_x) = nb.iter().position(|&x| x == leditnombre) {
                card_point += 1;
            }
        }
        for n in id+1..=id+card_point {
            if id == 1 {
                memory.insert(1, 0);
            }
            if let Some(&v) = memory.get(&n) {
                let mult = memory.get(&id).unwrap();
                memory.insert(n,  mult + v + 1);
            } else {
                memory.insert(n, 1);
            }
        };
        if card_point == 0 && !memory.contains_key(&id) {
            memory.insert(id, 0);
        }
        println!("{memory:?}");

        if card_point > 1 {
            card_point = 2_u32.pow(card_point - 1); 
        }
        
        _p1 += card_point;
    }
    // println!("total : {p1}");
    println!("memory : {:?}", memory.values().sum::<usize>() + memory.len());

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gab() {
        let input = 2;
        assert_eq!(input, 20117);
    }
}
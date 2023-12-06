use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let mut total_point = 0;
    for line in file.lines() {
        let (_card_id, other_line) = line.split_once(':').unwrap();
        let (winning, mycard) = other_line.split_once('|').unwrap();
        let nb: Vec<u32> = winning.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let my_nb: Vec<u32> = mycard.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        // println!("{my_nb:?}");
        let mut card_point: u32 = 0;

        for leditnombre in my_nb {
            if let Some(_x) = nb.iter().position(|&x| x == leditnombre) {
                card_point += 1;
            }
        }
        // println!("{card_id}");
        // println!("{other_line}");
        // println!("{nb:?}");
        // println!("cpbefore : {card_point}");

        if card_point > 1 {
            card_point = 2_u32.pow(card_point - 1); 
        }
        // println!("cpafter : {card_point}");
        
        total_point += card_point;
    }
    println!("total : {total_point}");

}

use std::{collections::HashMap, vec};
use aoc_runner_derive::aoc;

#[aoc(day4, part2)]
fn part_2(file: &str) -> Option<usize> {
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
    Some(memory.values().sum::<usize>() + oricard.len())

}

#[aoc(day4, part1)]
fn part_1(file: &str) -> Option<u32> {
    let mut total_point = 0;
    for line in file.lines() {
        let (_card_id, other_line) = line.split_once(':').unwrap();
        let (winning, mycard) = other_line.split_once('|').unwrap();
        let nb: Vec<u32> = winning.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let my_nb: Vec<u32> = mycard.trim().split(' ').filter(|x| !x.is_empty()).map(|c| c[0..c.len()].parse().unwrap()).collect();
        let mut card_point: u32 = 0;

        for leditnombre in my_nb {
            if let Some(_x) = nb.iter().position(|&x| x == leditnombre) {
                card_point += 1;
            }
        }
        if card_point > 1 {
            card_point = 2_u32.pow(card_point - 1); 
        }

        total_point += card_point;
    }
    Some(total_point)

}





#[cfg(test)]
mod tests {
    use super::*;

    const EXMPL: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL), Some(13))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL), Some(30))
    }
}

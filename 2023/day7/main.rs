use std::{fs, collections::HashMap, time::Instant, cmp::Ordering};

#[derive(Debug, Clone)]
struct Hand {
    ori_hand: String,
    card: HashMap<char, i32>, // The cards
    power: i32, // Power of all card
    kind: i32, // Ace or ..
    bid: i32,
}
impl Default for Hand {
    fn default() -> Self { Hand {card: HashMap::new(), power: 0, kind: 1, bid: 0, ori_hand: String::new()} }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.power.cmp(&other.power),
            Ordering::Less => self.kind.cmp(&other.kind),
            Ordering::Greater => self.kind.cmp(&other.kind),
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
impl Eq for Hand {}

fn main() {
    let cards: HashMap<char,i32> = HashMap::from([
        ('A',13),
        ('K',12),
        ('Q',11),
        ('J',10),
        ('T',9),
        ('9',8),
        ('8',7),
        ('7',6),
        ('6',5),
        ('5',4),
        ('4',3),
        ('3',2),
        ('2',1),
    ]);
    let now = Instant::now();
    let file = fs::read_to_string("input").unwrap();
    let mut arr: Vec<(&str, &str)> = vec![];
    let mut arr_of_card: Vec<Hand> = vec![];
    for line in file.lines() {
        arr.push(line.split_once(' ').unwrap());
    }
    for take in arr {
        // let mut p = 1;
        let mut h1: Hand = Hand {
            ori_hand: take.0.to_string(),
            .. Hand::default()
        };
        h1.bid = take.1.parse::<i32>().unwrap();
        for c in (take.0).chars() {
            h1.card.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
            // p += cards.get(&c).unwrap();
        }
        // h1.power = p;
        let mut p = 0;
        for occ in h1.card.values() {
            if *occ > p {
                p = *occ;
            }
        }
        
        match h1.card.len() {
            1 => h1.kind = 7,
            2 => {
                if p == 4 {
                    h1.kind = 6
                } else if p == 3 {
                    h1.kind = 5
                }
            },
            3 => {
                if p == 3 {
                    h1.kind = 4
                } else {
                    h1.kind = 3
                }
            }
            4 => h1.kind = 2,
            5 => h1.kind = 1,
            _ => (),
        }
        arr_of_card.push(h1);
    }
    let mut copy_arr = arr_of_card.clone();
    for (fidx, first_hand) in arr_of_card.clone().into_iter().enumerate() {
        for (cidx, compare_hand) in arr_of_card.clone().into_iter().enumerate() {
            if first_hand.kind == compare_hand.kind && first_hand.ori_hand != compare_hand.ori_hand {
                let mut f = first_hand.ori_hand.chars();
                let mut c = compare_hand.ori_hand.chars();
                loop {
                    let v1 = cards.get(&f.next().unwrap()).unwrap();
                    let v2 = cards.get(&c.next().unwrap()).unwrap();
                    match v1.cmp(v2) {
                        Ordering::Less => {
                            copy_arr[cidx].power += 1;
                            break;
                        },
                        Ordering::Equal => {
                            continue;
                        },
                        Ordering::Greater => {
                            copy_arr[fidx].power += 1;
                            break;
                        }
                    }
                
                }
            }
        }
    }

    // sorting <3
    copy_arr.sort();
    let mut result = 0;
    for (i, osef) in copy_arr.into_iter().enumerate() {
        result += osef.bid * (i as i32 + 1 )
    }

    println!("time: {}",now.elapsed().as_millis());

    // println!("{:#?}", copy_arr);
    println!("{}", result);
    // println!("cards: {:?}, power: {}, kind:{}", h1.card, h1.power, h1.kind);
}

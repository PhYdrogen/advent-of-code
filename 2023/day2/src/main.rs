use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut sum: Vec<u32> = vec![];
    let mut id:u32 = 1;

    for lines in input.lines() {

        let game_id: Vec<&str> = lines.split(':').collect();

        let sets: Vec<&str> = game_id[1].split(';').collect();
        let mut indivset: Vec<Vec<&str>> = vec![];
        for e in sets {
            let k: Vec<&str> = e.split(',').collect();
            indivset.push(k);
        };

        // println!("{:?}", game_id);
        // println!("{:?}", id);
        // // println!("{:?}", sets);
        // println!("{:?}", indivset);

        let mut possible: bool = true;

        for dices in indivset {
            for combo in dices {
                let i = combo.chars().nth(1).unwrap().to_digit(10).unwrap();
                let j = combo.chars().nth(2).unwrap().to_digit(10);
                let mut x = String::new();

                if let Some(nb) = j { 
                    x = format!("{}{}", i, nb)
                }
                let ccc = x.parse::<usize>();
                match ccc {
                    Ok(k) => {
                        if combo.contains("blue") && k > 14 || combo.contains("red") && k > 12 || combo.contains("green") && k > 13 {
                            possible = false;
                        } 
                    },
                    Err(_e) => (),
                }
            }
        }
        println!("for game {} it's possible ?: {}", id, possible);
        if possible {
            sum.push(id);
        }
        id += 1;
    }
    let aaa: u32 = sum.iter().sum();
    println!("{}", aaa);

}

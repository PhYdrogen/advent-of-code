use std::fs;

fn _part1(input: String) -> Vec<u32>{
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
    return sum
}

fn part2(input: String) -> Vec<u32> {
    let mut sum: Vec<u32> = vec![];
    // let mut id:u32 = 1;

    for lines in input.lines() {
        let mut color: Vec<(u32, &str)> = vec![
            (0, "red"),
            (0, "blue"),
            (0, "green"),
        ];
        let game_id: Vec<&str> = lines.split(':').collect();

        let sets: Vec<&str> = game_id[1].split(';').collect();
        let mut indivset: Vec<Vec<&str>> = vec![];
        for e in sets {
            let k: Vec<&str> = e.split(',').collect();
            indivset.push(k);
        };

        for &combo in &indivset[0] {
            // print!("{},", combo);  3 blue, 4 red, 1 blue, 2 green, 8 green, 6 blue, 20 red, 1 green, 3 red, 6 blue, 6 red, 1 blue, 3 green,0
            
            let i = combo.chars().nth(1).unwrap().to_digit(10).unwrap();
            let j = combo.chars().nth(2).unwrap().to_digit(10);

            let mut x = String::new();

            if let Some(nb) = j { 
                x = format!("{}{}", i, nb)
            }
            let okaaay = x.parse::<u32>();
            match okaaay {
                Ok(nb) => {
                    if combo.contains("red") && nb > color[0].0 {
                        color[0].0 = nb
                    }
                    if combo.contains("blue") && nb > color[1].0 {
                        color[1].0 = nb

                    }
                    if combo.contains("green") && nb > color[2].0 {
                        color[2].0 = nb
                    }
                },
                Err(_e) => (),
            }

        }
        let mult = color[0].0 * color[1].0 * color[2].0;
        sum.push(mult);
    } 

    sum
}


fn main() {
    let input = fs::read_to_string("input_test").unwrap();
    let result = part2(input);
    let nb: u32 = result.iter().sum();
    println!("{}", nb);

}

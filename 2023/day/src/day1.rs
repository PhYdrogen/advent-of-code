use aoc_runner_derive::aoc;


#[aoc(day1, part2)]
fn part_2(file: &str) -> Option<i32> {
    let arr: Vec<&str> = file.split_terminator('\n').collect();
    let mut secret: i32 = 0;
    for elem in arr {
        let mut sanitize = vec![];
        let mut construction = String::from(elem);
        {
            let _clean_string = elem;
            

            loop {
                let one = construction.find("one").unwrap_or(999);
                if one == 999 {
                    break;
                }
                construction = format!("{}1{}", &construction[..one+1], &construction[one+1..]);

            }
            loop {
                let two = construction.find("two").unwrap_or(999);
                if two == 999 {
                    break;
                } 
                construction = format!("{}2{}", &construction[..two+1], &construction[two+1..]);
            }
            loop {
                let three = construction.find("three").unwrap_or(999);
                if three == 999{
                    break;
                } 
                construction = format!("{}3{}", &construction[..three+1], &construction[three+1..]);
                
            }
            loop {
                let four = construction.find("four").unwrap_or(999);
                if four == 999 {
                    break;
                }
                construction = format!("{}4{}", &construction[..four+1], &construction[four+1..]);
                    
            }
            loop {
                let five = construction.find("five").unwrap_or(999);
                if five == 999 {
                    break;
                }
                construction = format!("{}5{}", &construction[..five+1], &construction[five+1..]);
                
            }
            loop {
                let six = construction.find("six").unwrap_or(999);
                if six == 999 {
                    break;
                }
                construction = format!("{}6{}", &construction[..six+1], &construction[six+1..]);

            }
            loop {
                let seven = construction.find("seven").unwrap_or(999);
                if seven == 999 {
                    break;
                } 
                construction = format!("{}7{}", &construction[..seven+1], &construction[seven+1..]);

            }
            loop {
                let eight = construction.find("eight").unwrap_or(999);
                if eight == 999 {
                    break;
                }
                construction = format!("{}8{}", &construction[..eight+1], &construction[eight+1..]);
                    
            }
            loop {
                let nine = construction.find("nine").unwrap_or(999);
                if nine == 999 {
                    break;
                }
                construction = format!("{}9{}", &construction[..nine+1], &construction[nine+1..]);
            }

        }

        for char in construction.chars() {
            if char.is_ascii_digit() {
                sanitize.push(char);
            }
        }
        let calc: i32;
        if sanitize.len() > 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.last().unwrap().to_string());
            calc = k.parse::<i32>().unwrap();
        }
        else if sanitize.len() == 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.first().unwrap().to_string());
            calc = k.parse::<i32>().unwrap();
        } else {
            calc = 0;
        }
        secret += calc;
    }
    println!("The secret count is {secret}");
    Some(secret)
}

#[aoc(day1, part1)]
fn part_1(file: &str) -> Option<i32> {
    //read file
    // let content = fs::read_to_string("input").expect("Erreur");
    //convert string to arr of string
    let arr: Vec<&str> = file.split_terminator('\n').collect();
    let mut secret: i32 = 0;
    for elem in arr {
        let mut sanitize = vec![];
        for char in elem.chars() {
            if char.is_ascii_digit() {
                sanitize.push(char);
            }
        }
        print!("{sanitize:?} ");
        let calc: i32;
        if sanitize.len() > 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.last().unwrap().to_string());
            calc = k.parse::<i32>().unwrap();
        }
        else if sanitize.len() == 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.first().unwrap().to_string());
            calc = k.parse::<i32>().unwrap();
        } else {
            calc = 0;
        }
        println!("{calc:?}");
        secret += calc;
    }
    println!("The secret count is {secret}");
    Some(secret)
}

#[cfg(test)]
mod test {
    use super::*;
    const EXMPL_P1:&str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL_P1), Some(53974))
    }

    const EXMPL_P2:&str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL_P2), Some(52840))
    }
}
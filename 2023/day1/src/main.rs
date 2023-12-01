use std::fs;

fn main() {
    //read file
    let content = fs::read_to_string("input").expect("Erreur");
    //convert string to arr of string
    let arr: Vec<&str> = content.split_terminator('\n').collect();
    let mut secret: usize = 0;
    for elem in arr {
        let mut sanitize = vec![];
        let mut construction = String::from(elem);
        {
            let clean_string = elem;
            
            let one = elem.find("one").unwrap_or(0);
            let two = elem.find("two").unwrap_or(0);
            let three = elem.find("three").unwrap_or(0);
            let four = elem.find("four").unwrap_or(0);
            let five = elem.find("five").unwrap_or(0);
            let six = elem.find("six").unwrap_or(0);
            let seven = elem.find("seven").unwrap_or(0);
            let eight = elem.find("eight").unwrap_or(0);
            let nine = elem.find("nine").unwrap_or(0);

            if one != 0 {
                construction = format!("{}1{}", &clean_string[..one], &clean_string[one+1..]);
            }
            if two != 0 && construction.is_empty() {
                construction = format!("{}2{}", &clean_string[..two], &clean_string[two+1..]);
            } else if two != 0 && !construction.is_empty() {
                construction = format!("{}2{}", &construction[..two], &construction[two+1..]);

            }
            if three != 0 && construction.is_empty() {
                construction = format!("{}3{}", &clean_string[..three], &clean_string[three+1..]);
            } else if three != 0 && !construction.is_empty() {
                construction = format!("{}3{}", &construction[..three], &construction[three+1..]);

            }
            if four != 0 && construction.is_empty() {
                construction = format!("{}4{}", &clean_string[..four], &clean_string[four+1..]);
            } else if four != 0 && !construction.is_empty() {
                construction = format!("{}4{}", &construction[..four], &construction[four+1..]);

            }
            if five != 0 && construction.is_empty() {
                construction = format!("{}5{}", &clean_string[..five], &clean_string[five+1..]);
            } else if five != 0 && !construction.is_empty() {
                construction = format!("{}5{}", &construction[..five], &construction[five+1..]);

            }
            if six != 0 && construction.is_empty() {
                construction = format!("{}6{}", &clean_string[..six], &clean_string[six+1..]);
            } else if six != 0 && !construction.is_empty() {
                construction = format!("{}6{}", &construction[..six], &construction[six+1..]);
                
            }
            if seven != 0 && construction.is_empty(){
                construction = format!("{}7{}", &clean_string[..seven], &clean_string[seven+1..]);
            } else if seven != 0 && !construction.is_empty() {
                construction = format!("{}7{}", &construction[..seven], &construction[seven+1..]);

            }
            if eight != 0 && construction.is_empty() {
                construction = format!("{}8{}", &clean_string[..eight], &clean_string[eight+1..]);
            } else if eight != 0 && !construction.is_empty() {
                construction = format!("{}8{}", &construction[..eight], &construction[eight+1..]);

            }
            if nine != 0 && construction.is_empty() {
                construction = format!("{}9{}", &clean_string[..nine], &clean_string[nine+1..]);
            } else if nine != 0 && !construction.is_empty() {
                construction = format!("{}9{}", &construction[..nine], &construction[nine+1..]);

            }
            println!("\nori : {}\n constr : {}", elem, construction);

        }

        for char in construction.chars() {
            if char.is_ascii_digit() {
                sanitize.push(char);
            }
        }
        print!("{sanitize:?} ");
        let calc: usize;
        if sanitize.len() > 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.last().unwrap().to_string());
            calc = k.parse::<usize>().unwrap();
        }
        else if sanitize.len() == 1 {
            let k = format!("{}{}", sanitize.first().unwrap().to_string(), sanitize.first().unwrap().to_string());
            calc = k.parse::<usize>().unwrap();
        } else {
            calc = 0;
        }
        println!("{calc:?}");
        secret += calc;
    }
    println!("The secret count is {secret}");
}

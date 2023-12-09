use std::fs;

fn main() {
    //read file
    let content = fs::read_to_string("input_test").expect("Erreur");
    //convert string to arr of string
    let arr: Vec<&str> = content.split_terminator('\n').collect();
    let mut secret: usize = 0;
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

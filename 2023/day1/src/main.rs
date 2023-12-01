use std::fs;

#[warn(clippy::comparison_chain)]
fn main() {
    //read file
    let content = fs::read_to_string("input").expect("Erreur");
    //convert string to arr of string
    let arr: Vec<&str> = content.split_terminator('\n').collect();
    let mut secret: usize = 0;
    for elem in arr {
        let mut sanitize = vec![];
        let mut duo: Vec<String> = vec![];
        for char in elem.chars() {
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

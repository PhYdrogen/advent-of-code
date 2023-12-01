use std::fs;

fn main() {
    //read file
    let content = fs::read_to_string("input").expect("Erreur");
    //convert string to arr of string
    let arr: Vec<&str> = content.split_terminator('\n').collect();
    let mut todo: Vec<String> = vec![];
    for elem in arr {
        let mut sanitize = vec![];
        for char in elem.chars() {
            if char.is_ascii_digit() {
                sanitize.push(char);
            }
        }
        print!("{sanitize:?}");
        if sanitize.len() > 1 {
            todo.push(sanitize.first().unwrap().to_string());
            todo.push(sanitize.last().unwrap().to_string());
        }
        println!("{todo:?}");
        // return
    }
}

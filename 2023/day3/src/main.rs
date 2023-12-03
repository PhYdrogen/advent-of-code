use std::fs;

fn main() {
    let symbol = ['$','*','+','-','=','%','@','/','#'];
    let mut bomb = vec![];
    
    let file = fs::read_to_string("input_test").unwrap();
    for (index, line) in file.lines().enumerate() {
        for &sy in symbol.iter() {
            if line.contains(sy) && index > 0 {
                let pos = line.find(sy).unwrap();
                println!("find {sy} at {index}:{pos}");
                bomb.push((index, pos));
            }
        }
    }
    
    let mut pos_nb = vec![];

    for line in file.lines() {
        let mut number = String::new();

        for (j, l) in line.chars().enumerate() {
            if l.is_ascii_digit() {
                print!("{l}");
                number = format!("{number}{l}");
            }
            if l == '.' && !number.is_empty() {
                let nb = number.parse::<usize>().unwrap();
                pos_nb.push((j - number.len(), j, nb));
                number = String::new();
            }
        }
    }
    println!("possible: {pos_nb:?}")

}

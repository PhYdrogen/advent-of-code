use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Erreur");
    println!("{content}");
}

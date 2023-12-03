use std::fs;

fn main() {
    let file = fs::read_to_string("input_test").unwrap();
    let t = file.lines().map(|f| {
        f
    }).collect();
    println!("{t}");

}

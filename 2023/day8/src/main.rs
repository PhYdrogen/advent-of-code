use std::{iter, fs, collections::HashMap};
fn main() {
    let file = fs::read_to_string("input").unwrap();
    let (instruction, file) = file.split_once('\n').unwrap();
    let mut map = HashMap::new();
    for line in file.lines() {
        if !line.is_empty() {
            let x = line.split_once(" = ").unwrap();
            let y = x.1.split_once(", ").unwrap();
            map.insert(x.0, (&y.0[1..], &y.1[0..3]));
        }
    }
    
    let mut cursor = map.get("AAA").unwrap();
    let mut steps = 0;
    let mut found = false;
    // we follow instr
    let none_finite_instruction = iter::repeat(instruction);
    for instruction in none_finite_instruction {
        for ins in instruction.chars() {
            if ins == 'R' {
                if cursor.1 == "ZZZ" {
                    steps += 1;
                    found = true;
                    break;
                }
                cursor = map.get(&cursor.1).unwrap();
                steps += 1;
            } else {
                if cursor.0 == "ZZZ" {
                    steps += 1;
                    found = true;
                    break;
                }
                cursor = map.get(&cursor.0).unwrap();
                steps += 1;
            }
        }
        if found {
            break;
        }
    }

    println!("{:?}\n{}\n{}", map, instruction, steps);
}

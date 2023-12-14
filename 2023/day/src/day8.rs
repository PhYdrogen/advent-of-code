use std::{collections::HashMap, fs, iter, time::Instant};

pub fn part_1() {

}

pub fn part_2() {
    let now = Instant::now();
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

    let mut key_arr = vec![];
    for key in map.keys() {
        if key.ends_with('A') {
            key_arr.push(key)
        }
    }
    let mut cursor_arr = key_arr
        .iter()
        .map(|&e| map.get(e).unwrap().to_owned())
        .collect::<Vec<(&str, &str)>>();
    let c_len = cursor_arr.len();
    // let mut cursor = map.get("AAA").unwrap();
    let mut steps: usize = 0;
    let mut found = false;
    println!("{:?}", cursor_arr);
    // we follow instr
    let none_finite_instruction = iter::repeat(instruction);
    for instruction in none_finite_instruction {
        if steps % 10_000 == 0 {
            println!("{steps}");
        }
        for ins in instruction.chars() {
            let mut check = 0;
            if ins == 'R' {
                for cursor in &mut cursor_arr {
                    if cursor.1.ends_with('Z') {
                        check += 1;
                        if check == c_len {
                            found = true;
                            break;
                        }
                    }
                    *cursor = *map.get(cursor.1).unwrap();
                }
                steps += 1;
            } else {
                for cursor in &mut cursor_arr {
                    if cursor.0.ends_with('Z') {
                        check += 1;
                        if check == c_len {
                            found = true;
                            break;
                        }
                    }
                    *cursor = *map.get(cursor.0).unwrap();
                }
                steps += 1;
            }
        }
        if found {
            break;
        }
    }

    println!(
        "{:?}\n{}\nsteps: {}\ntime:{}",
        map,
        instruction,
        steps,
        now.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use crate::day8::*;

    #[test]
    fn test_part_1() {

    }
    // Part2 is not optimize for brute force (normal way)
    #[test]
    fn test_part_2() {
        part_2()
    }
}
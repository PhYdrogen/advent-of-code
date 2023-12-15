use aoc_runner_derive::aoc;
use std::{collections::HashMap, iter, time::Instant};

#[aoc(day8, part1)]
fn part_1(file: &str) -> Option<i32> {
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
    Some(steps)
}

pub fn part_2(file: &str) -> Option<usize> {
    let now = Instant::now();
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
    Some(steps)
}

#[cfg(test)]
mod test {
    use crate::day8::*;

    const EXMPL_P1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL_P1), Some(6));
    }
    const EXMPL_P2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    // Part2 is not optimize for brute force (normal way)
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL_P2), Some(6));
    }
}

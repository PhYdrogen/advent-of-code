use std::fs;

fn parse(file: String) -> Vec<i64> {
    let mut seeds_arr: Vec<i64> = vec![];
    let mut toxicseed: (i64, bool);
    let mut case_arr = vec![];

    for line in file.lines() {
        if line.contains("seeds") {
            let k = line.split_once("seeds: ").unwrap().1;
            seeds_arr = k.split(' ').map(|e: &str| e.parse().unwrap()).collect();
        }
    }
    for s in seeds_arr {
        toxicseed = (s, false);
        for line in file.lines() {
            if !line.contains("map:") && !line.contains("seeds:") && !line.is_empty() {
                // println!("la line: {}", line);
                let line_arr_num: Vec<i64> =
                    line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
                if line_arr_num[1] <= toxicseed.0
                    && (line_arr_num[1] + line_arr_num[2]) >= toxicseed.0
                    && !toxicseed.1
                {
                    let diff = line_arr_num[0] - line_arr_num[1];
                    // print!(" {} ->", toxicseed.0);
                    toxicseed.0 += diff;
                    toxicseed.1 = true;
                    // print!("{} ", toxicseed.0);
                }
            } else {
                toxicseed.1 = false;
            }
        }
        case_arr.push(toxicseed.0);
    }
    case_arr
}
fn part_1() {}
fn main() {
    let a = parse(fs::read_to_string("input").unwrap());
    println!("p1: {}", a.iter().min().unwrap());
}

use rayon::prelude::*;
use std::fs;

fn parse(file: String) -> Vec<i64> {
    let mut seeds_arr: Vec<i64> = vec![];

    for line in file.lines() {
        if line.contains("seeds") {
            let k = line.split_once("seeds: ").unwrap().1;
            seeds_arr = k.split(' ').map(|e: &str| e.parse().unwrap()).collect();
        }
    }
    seeds_arr
}
fn part_1(seeds_arr: Vec<i64>, mut arr: Vec<i64>, file: String) -> Vec<i64> {
    seeds_arr.into_iter().enumerate().for_each(|(idx, s)| {
        let mut toxicseed: (i64, bool);

        toxicseed = (s, false);
        for line in file.lines() {
            if !line.contains("map:") && !line.contains("seeds:") && !line.is_empty() {
                let line_arr_num: Vec<i64> =
                    line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
                if line_arr_num[1] <= toxicseed.0 && (line_arr_num[1] + line_arr_num[2]) >= toxicseed.0 && !toxicseed.1 {
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
        // println!("{}", toxicseed.0);
        arr.push(toxicseed.0);
        if idx % 100_000 == 0 {
            println!("{}", idx / 100_000);
        }
        // toxicseed.0
    });
    // seeds_arr

    arr
}
fn part_2(file: String) -> Vec<i64> {
    let mut seeds_arr: Vec<i64> = vec![];
    let mut multi_seeds: Vec<i64> = vec![];

    for line in file.lines() {
        if line.contains("seeds") {
            let k = line.split_once("seeds: ").unwrap().1;
            seeds_arr = k.split(' ').map(|e: &str| e.parse().unwrap()).collect();
        }
    }

    for seed in seeds_arr[0]..seeds_arr[0] + seeds_arr[1] {
        multi_seeds.push(seed);
    }
    println!("f1 done: {} elem", multi_seeds.len());

    for seed in seeds_arr[2]..seeds_arr[2] + seeds_arr[3] {
        multi_seeds.push(seed);
    }
    println!("f1 done: {} elem", multi_seeds.len());
    multi_seeds
}
fn main() {
    let filename = "input";
    let arr: Vec<i64> = vec![];
    let a = part_2(fs::read_to_string(filename).unwrap());
    let b = part_1(a, arr, fs::read_to_string(filename).unwrap());
    println!("p1: {}", b.iter().min().unwrap());
}

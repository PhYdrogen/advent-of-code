use rayon::prelude::*;
use std::fs;

fn balade(tseed: &mut (i64,bool), file: String) -> i64 {
    for line in file.lines() {
        if !line.contains("map:") && !line.contains("seeds:") && !line.is_empty() {
            let line_arr_num: Vec<i64> =
                line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
            if line_arr_num[1] <= tseed.0 && (line_arr_num[1] + line_arr_num[2]) >= tseed.0 && !tseed.1 {
                let diff = line_arr_num[0] - line_arr_num[1];
                print!(" {} ->", tseed.0);
                tseed.0 += diff;
                tseed.1 = true;
                print!("{} ", tseed.0);
            }
        } else {
            tseed.1 = false;
        }
    }
    tseed.0
}

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
fn part_1(mut seeds_arr: Vec<i64>, file: String) -> Vec<i64> {

    seeds_arr.par_iter_mut().for_each(|s| {
        let mut toxicseed: (i64, bool) = (*s, false);
        *s = balade(&mut toxicseed, file.clone());

    });
    seeds_arr
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
    let b = part_1(a, fs::read_to_string(filename).unwrap());
    println!("p1: {}", b.into_iter().min().unwrap());
}

use rayon::{
    self,
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
};
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
fn part_1(seeds_arr: Vec<i64>, file: String) -> Vec<i64> {
    let a = seeds_arr.clone().into_par_iter().for_each(move |s| {
        let mut toxicseed: (i64, bool) = (0, false);
        // let mut case_arr = vec![];

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
        println!("{}", toxicseed.0);
        // case_arr.push(toxicseed.0);
        // toxicseed.0
    });
    seeds_arr

    // case_arr
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
    let filename = "input_test";
    let a = part_2(fs::read_to_string(filename).unwrap());
    let b = part_1(a, fs::read_to_string(filename).unwrap());
    println!("p1: {}", b.iter().min().unwrap());
}

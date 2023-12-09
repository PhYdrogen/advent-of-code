use rayon::prelude::*;
use std::{time::Instant, fs, collections::HashMap, borrow::BorrowMut};

fn create_chunk(file: String) -> HashMap<i32, Vec<Vec<i64>>> {
    let mut chunk: HashMap<i32, Vec<Vec<i64>>> = HashMap::new(); // { 0: [[50, 98, 2], [52, 50, 48]] },{ 1: [[0, 15, 37], [37 52 2] ... ] } // chunk[1][1]
    let mut counter = 0;
    let mut chunk_arr: Vec<Vec<i64>> = vec![];

    for line in file.lines() {
        if !line.contains("map:") && !line.contains("seeds:") && !line.is_empty() {
            let line_arr_num: Vec<i64> = line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
            chunk_arr.push(line_arr_num);
        } else if !chunk_arr.is_empty() {
            chunk.insert(counter, chunk_arr.clone());
            chunk_arr = vec![];
            counter += 1;
        }
    }
    chunk
}


fn balade(tseed: i64, chunk: HashMap<i32, Vec<Vec<i64>>>) -> i64 {
    let mut par_seed = tseed;
    for c in 0..chunk.len() {
        let chunk_arr = chunk.get(&(c as i32)).unwrap();
        for line_arr_num in chunk_arr.iter() {
            if line_arr_num[1] <= par_seed && (line_arr_num[1] + line_arr_num[2]) >= par_seed {
                let diff = line_arr_num[0] - line_arr_num[1];
                // print!(" {} ->", tseed);
                par_seed += diff;
                // println!("{} ", tseed);
                break;
            }
        }
    }
    par_seed
}

fn _parse(file: String) -> Vec<i64> {
    let mut seeds_arr: Vec<i64> = vec![];

    for line in file.lines() {
        if line.contains("seeds") {
            let k = line.split_once("seeds: ").unwrap().1;
            seeds_arr = k.split(' ').map(|e: &str| e.parse().unwrap()).collect();
        }
    }
    seeds_arr
}
fn part_1(seeds_arr: Vec<(i64, i64)>, chunk: HashMap<i32, Vec<Vec<i64>>>) -> i64 {
    let mut final_arr: Vec<i64> = vec![];

    seeds_arr.par_iter().for_each(|s| {
        println!("s: {:?}", s);
        (s.0..=s.1).into_par_iter().for_each(|l| {
            final_arr.push(balade(l, chunk.clone()));
        })
    });
    final_arr.into_iter().min().unwrap()
}
fn part_2(file: String) -> Vec<(i64, i64)> {
    let mut seeds_arr: Vec<i64> = vec![];
    let mut multi_seeds: Vec<_> = vec![];

    for line in file.lines() {
        if line.contains("seeds") {
            let k = line.split_once("seeds: ").unwrap().1;
            seeds_arr = k.split(' ').map(|e: &str| e.parse().unwrap()).collect();
        }
    }

    for (i, s) in seeds_arr.iter().enumerate() {
        if i % 2 == 0 {
            multi_seeds.push((*s, s + seeds_arr[i + 1]));
            multi_seeds.sort();
        }
    }

    for (d, f) in multi_seeds.clone().iter() {
        for (j, k) in multi_seeds.clone().iter() {
            if d < j && j < f && d < k && k < f {
                if let Some(target) = multi_seeds.par_iter().position_any(|&elem| elem.0 == *j) {
                    multi_seeds.remove(target);
                }
            }
            if d < j && j < f && d < k && f < k {
                let t1 = multi_seeds.par_iter().position_any(|&elem| elem.0 == *d).unwrap();
                let t2 = multi_seeds.par_iter().position_any(|&elem| elem.0 == *j).unwrap();
                multi_seeds[t1].1 = *k;
                multi_seeds.remove(t2);

            }
        }
    }
    // panic!("stop! {:?}", multi_seeds);
    multi_seeds

}
fn main() {
    let filename = "input_gab";
    let chunk = create_chunk(fs::read_to_string(filename).unwrap());
    let n = Instant::now();
    let a = part_2(fs::read_to_string(filename).unwrap());
    let b = part_1(a, chunk);
    println!("p1: {}, time: {}", b, n.elapsed().as_secs());
}

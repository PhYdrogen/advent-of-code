use rayon::prelude::*;
use std::fs;

fn balade(tseed: &mut (i64,bool), file: String) -> i64 {
    for line in file.lines() {
        if !line.contains("map:") && !line.contains("seeds:") && !line.is_empty() {
            let line_arr_num: Vec<i64> =
                line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
            if line_arr_num[1] <= tseed.0 && (line_arr_num[1] + line_arr_num[2]) >= tseed.0 && !tseed.1 {
                let diff = line_arr_num[0] - line_arr_num[1];
                // print!(" {} ->", tseed.0);
                tseed.0 += diff;
                tseed.1 = true;
                // print!("{} ", tseed.0);
            }
        } else {
            tseed.1 = false;
        }
    }
    tseed.0
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
fn part_1(mut seeds_arr: Vec<(i64, i64)>, file: String) -> i64 {
    let mut final_arr: Vec<i64> = vec![];

    seeds_arr.iter_mut().for_each(|s| {
        (s.0..=s.1).for_each(|l| {
            final_arr.push(balade(&mut (l, false), file.clone()));
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
    multi_seeds
    // panic!("stop! {:?}", multi_seeds);

}
fn main() {
    let filename = "input_test";
    let a = part_2(fs::read_to_string(filename).unwrap());
    let b = part_1(a, fs::read_to_string(filename).unwrap());
    println!("p1: {}", b);
}

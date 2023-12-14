use rayon::prelude::*;
use aoc_runner_derive::aoc;

use self::parser::{farmer, create_chunk};

#[aoc(day5, part2)]
fn part_2(file: &str) -> Option<i64> {
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
    Some(farmer(multi_seeds, create_chunk(file)))

}

#[aoc(day5, part1)]
fn part_1(file: &str) -> Option<i64> {
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
                let line_arr_num: Vec<i64> =
                    line.split(' ').map(|c: &str| c.parse().unwrap()).collect();
                if line_arr_num[1] <= toxicseed.0
                    && (line_arr_num[1] + line_arr_num[2]) >= toxicseed.0
                    && !toxicseed.1
                {
                    let diff = line_arr_num[0] - line_arr_num[1];
                    toxicseed.0 += diff;
                    toxicseed.1 = true;
                }
            } else {
                toxicseed.1 = false;
            }
        }
        case_arr.push(toxicseed.0);
    }
    Some(*case_arr.iter().min().unwrap())
}


mod parser {
    use rayon::prelude::*;
    use std::collections::HashMap;

    pub fn farmer(seeds_arr: Vec<(i64, i64)>, chunk: HashMap<i32, Vec<Vec<i64>>>) -> i64 {
        let final_arr: Vec<i64> = seeds_arr
            .par_iter()
            .flat_map(|s| {
                (s.0..=s.1).into_par_iter().map(|l| {
                    worker(l, chunk.clone())
                }).collect::<Vec<i64>>()
            })
            .collect();
        final_arr.into_iter().min().unwrap()
    }

    pub fn worker(tseed: i64, chunk: HashMap<i32, Vec<Vec<i64>>>) -> i64 {
        let mut par_seed = tseed;
        for c in 0..chunk.len() {
            let chunk_arr = chunk.get(&(c as i32)).unwrap();
            for line_arr_num in chunk_arr.iter() {
                if line_arr_num[1] <= par_seed && (line_arr_num[1] + line_arr_num[2]) >= par_seed {
                    let diff = line_arr_num[0] - line_arr_num[1];
                    par_seed += diff;
                    break;
                }
            }
        }
        par_seed
    }

    pub fn create_chunk(file: &str) -> HashMap<i32, Vec<Vec<i64>>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXMPL: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL), Some(35))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL), Some(46))
    }
}

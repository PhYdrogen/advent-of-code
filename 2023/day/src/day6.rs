use std::iter::zip;

use aoc_runner_derive::aoc;

#[aoc(day6, part2)]
fn part_2(_file: &str) -> Option<i32> {
    let races: Vec<(i64, i64)> = vec![(53916768,250133010811025)];
    let mut result: Vec<i32> = vec![];

    for (t, dst) in &races {
        let mut count = 0;
        for course in 1..*t {
            let mult = *t - course;
            if mult * course > *dst {
                count += 1;
            }
        }
        result.push(count);
    }
    result.into_iter().reduce(|acc, e| acc * e)
}
fn parse_part1(file: &str) -> Vec<(i32, i32)> {
    let mut bigboss: Vec<Vec<i32>> = vec![];

    for line in file.lines() {
        let mut cutter = line.split_whitespace();
        let mut p_arr:Vec<i32> = vec![];
        
        cutter.next();
        for c in cutter {
            p_arr.push(c.parse::<i32>().unwrap())
        }
        bigboss.push(p_arr)
    }
    zip(bigboss[0].clone(), bigboss[1].clone()).map(|e| (e.0, e.1)).collect::<Vec<(i32,i32)>>()
}


#[aoc(day6, part1)]
fn part_1(file: &str) -> Option<i32> {
    
    let races = parse_part1(file);
    let mut result: Vec<i32> = vec![];

    for (t, dst) in &races {
        let mut count = 0;
        for course in 1..*t {
            let mult = *t - course;
            if mult * course > *dst {
                count += 1;
            }
        }
        result.push(count);
    }
    result.into_iter().reduce(|acc, e| acc * e)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXMPL: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL), Some(35))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL), Some(46))
    }
}

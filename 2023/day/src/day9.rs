use aoc_runner_derive::aoc;
use rayon::iter::IndexedParallelIterator;

#[aoc(day9, part1)]
fn part_1(file: &str) -> Option<i32> {

    for arr in file_parser(file) {
        walker(arr, 0);
    }

    None
}
fn walker(arr: Vec<i32>, cpt: i32) -> i32 {
    let mut v = vec![];

    if arr.iter().sum::<i32>() == 0 {
        return *arr.last().unwrap();
    }

    for idx in (0..arr.len() - 1).step_by(2) {
        v.push(arr[idx] - arr[idx+1])
    }
    println!("recur !");
    walker(v, cpt) + cpt
}

fn file_parser(file: &str) -> Vec<Vec<i32>> {
    file.lines().map(|f| {
        f.split(' ').map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }).collect()
}

#[aoc(day9, part2)]
fn part_2(file: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod test {

    use super::*;
    const EXMPL: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL), Some(114));
    }


    // Part2 is not optimize for brute force (normal way)
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL), Some(0));

    }
}
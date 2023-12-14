use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
fn part_1(file: &str) -> Option<i32> {
    let mut ye = vec![];

    for arr in file_parser(file) {
        walker(arr, 0);
    }
    Some(ye.iter().sum())
}
fn walker(arr: Vec<i32>, cpt: i32) -> i32 {
    let mut v = vec![];

    print!("{:?}", arr);
    if arr.iter().sum::<i32>() == 0 {
        return arr.last().unwrap().clone();
    }

    for idx in 0..arr.len() - 1 {
        v.push(arr[idx+1] - arr[idx])
    }
    println!("recur ! cpt: {}", cpt);
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
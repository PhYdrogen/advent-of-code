use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
fn part_1(file: &str) -> Option<i32> {
    None
}

#[aoc(day9, part2)]
fn part_2(file: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod test {

    use super::*;
    const EXMPL: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXMPL), Some(0));
    }


    // Part2 is not optimize for brute force (normal way)
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXMPL), Some(0));

    }
}
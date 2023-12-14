use std::collections::HashMap;
use aoc_runner_derive::aoc;

struct Bomb {
    x: i32,
    y: i32,
}

#[aoc(day3, part2)]
fn part_2(file: &str) -> Option<usize> {
    let part_1 = false;

    let bomb = create_bomb(part_1, file);

    let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

    let hmap = check_cell(part_1, bomb, pos_nb);

    Some(hmap.values().sum::<usize>())

}

#[aoc(day3, part1)]
fn part_1(file: &str) -> Option<usize> {
    let part_1 = true;

    let bomb = create_bomb(part_1, file);

    let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

    let hmap = check_cell(part_1, bomb, pos_nb);

    Some(hmap.values().sum::<usize>())
}

fn create_bomb(part_1: bool, file: &str) -> Vec<Bomb> {

    // thanks rust i like
    let symbol = if part_1 {
        vec!['$', '*', '+', '-', '=', '%', '@', '/', '#', '&']
    } else {
        vec!['*']
    };

    let mut arr: Vec<Bomb> = vec![];

    for (index, line) in file.lines().enumerate() {
        for c in line.chars().enumerate() {
            for &sy in symbol.iter() {
                if c.1 == sy {
                    println!("{sy} at {index}:{}", c.0);
                    arr.push(Bomb {x: index as i32, y: c.0 as i32});
                }
            }
        }
    }
    arr
}

fn localiser_chiffre(file: &str) -> Vec<(i32, i32, i32, usize)> {
    let mut pos_nb: Vec<(i32, i32, i32, usize)> = vec![];

    for (line_count, line) in file.lines().enumerate() {
        let mut number = String::new();

        for (j, l) in line.chars().enumerate() {
            if l.is_ascii_digit() {
                number = format!("{number}{l}");
            }
            if !l.is_ascii_digit() && !number.is_empty()
                || !number.is_empty() && line.len() - 1 == j
            {
                let nb = number.parse::<usize>().unwrap();
                // println!("j:{}, l:{}",j,l);

                pos_nb.push((
                    line_count as i32,
                    (j - number.len()) as i32,
                    (j - 1) as i32,
                    nb,
                ));
                number = String::new();
            }
        }
    }
    pos_nb
}

fn check_cell(
    part_1: bool,
    arr: Vec<Bomb>,
    bigtuple: Vec<(i32, i32, i32, usize)>,
) -> HashMap<(i32, i32), usize> {
    let mut hash: HashMap<(i32, i32), usize> = HashMap::new();
    let mut pash: HashMap<(i32, i32), usize> = HashMap::new();

    for b in arr {
        let mut found = 0;
        hash.clear();
        for (ligne_chiffre, debut_chiffre, fin_chiffre, leditchiffre) in bigtuple.iter() {
            if b.x == *ligne_chiffre {
                if b.y == *debut_chiffre - 1 {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                    found += 1;
                }
                if b.y == *fin_chiffre + 1 {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                    found += 1;
                }
                if b.y == *debut_chiffre {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                    found += 1;
                }
            }
            if b.x - 1 == *ligne_chiffre
                && b.y >= *debut_chiffre - 1
                && b.y <= *fin_chiffre + 1
            {
                hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                found += 1;
            }
            if b.x + 1 == *ligne_chiffre
                && b.y >= *debut_chiffre - 1
                && b.y <= *fin_chiffre + 1
            {
                hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                found += 1;
            }
            println!("{found}: {:?}",hash.values());
            if found == 2 {
                let mut calc = vec![];
                for v in hash.values() {
                    calc.push(v);
                }
                pash.insert((*ligne_chiffre, *fin_chiffre), calc[0] * calc[1]);
                found = 0;
            }
        }
    }
    

    if part_1 {
        hash 
    } else {
        pash
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXMPL_P1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        let part_1 = true;

        let bomb = create_bomb(part_1, EXMPL_P1);

        let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(EXMPL_P1);

        let hmap = check_cell(part_1, bomb, pos_nb);

        let su: usize = hmap.values().sum::<usize>();
        assert_eq!(su, 4361)
    }

    #[test]
    fn test_part_2() {
        let part_1 = false;

        let bomb = create_bomb(part_1, EXMPL_P1);

        let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(EXMPL_P1);

        let hmap = check_cell(part_1, bomb, pos_nb);

        let su: usize = hmap.values().sum::<usize>();
        assert_eq!(su, 467835)
    }
}

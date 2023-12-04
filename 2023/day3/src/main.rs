use std::{collections::HashMap, fs};

fn create_bomb(file: String) -> Vec<(i32, i32)> {
    let symbol = ['$', '*', '+', '-', '=', '%', '@', '/', '#', '&'];
    let mut arr: Vec<(i32, i32)> = vec![];

    for (index, line) in file.lines().enumerate() {
        for c in line.chars().enumerate() {
            for &sy in symbol.iter() {
                if c.1 == sy {
                    println!("{sy} at {index}:{}", c.0);
                    arr.push((index as i32, c.0 as i32));
                }
            }
        }
    }
    arr
}

fn localiser_chiffre(file: String) -> Vec<(i32, i32, i32, usize)> {
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
    arr: Vec<(i32, i32)>,
    bigtuple: Vec<(i32, i32, i32, usize)>,
) -> HashMap<(i32, i32), usize> {
    let mut hash: HashMap<(i32, i32), usize> = HashMap::new();

    for (ligne_signe, debut_signe) in arr {
        for (ligne_chiffre, debut_chiffre, fin_chiffre, leditchiffre) in bigtuple.iter() {
            if ligne_signe == *ligne_chiffre {
                if debut_signe == *debut_chiffre - 1 {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                }
                if debut_signe == *fin_chiffre + 1 {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                }
                if debut_signe == *debut_chiffre {
                    hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
                }
            }
            if ligne_signe - 1 == *ligne_chiffre
                && debut_signe >= *debut_chiffre - 1
                && debut_signe <= *fin_chiffre + 1
            {
                hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
            }
            if ligne_signe + 1 == *ligne_chiffre
                && debut_signe >= *debut_chiffre - 1
                && debut_signe <= *fin_chiffre + 1
            {
                hash.insert((*ligne_chiffre, *fin_chiffre), leditchiffre.to_owned());
            }
        }
    }
    hash
}

fn main() {
    let file_name: &str = "input";

    let file = fs::read_to_string(file_name).unwrap();
    let bomb = create_bomb(file.clone());

    let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

    // println!("{pos_nb:?}");
    let hmap = check_cell(bomb, pos_nb);

    // println!("{hmap:?}");
    let su: usize = hmap.values().sum::<usize>();
    println!("final: {:?}", su);
}

#[cfg(test)]
mod tests {
    use crate::{check_cell, create_bomb, localiser_chiffre};
    use std::fs;
    #[test]
    fn input_gab() {
        let file_name: &str = "input_gab";

        let file = fs::read_to_string(file_name).unwrap();
        let bomb = create_bomb(file.clone());

        let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

        println!("{pos_nb:?}");
        let hmap = check_cell(bomb, pos_nb);

        println!("{hmap:?}");
        let mut idk: Vec<_> = hmap.values().collect();
        idk.sort();
        println!("{:?}", idk);
        let su: usize = hmap.values().sum::<usize>();
        println!("final: {:?}", su);
        assert_eq!(su, 1782)
    }
    #[test]
    fn input_test() {
        let file_name: &str = "input_test";

        let file = fs::read_to_string(file_name).unwrap();
        let bomb = create_bomb(file.clone());

        let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

        println!("{pos_nb:?}");
        let hmap = check_cell(bomb, pos_nb);

        println!("{hmap:?}");
        let su: usize = hmap.values().sum::<usize>();
        println!("final: {:?}", su);
        assert_eq!(su, 4361)
    }

    #[test]
    fn input_more() {
        let file_name: &str = "input_more";

        let file = fs::read_to_string(file_name).unwrap();
        let bomb = create_bomb(file.clone());

        let pos_nb: Vec<(i32, i32, i32, usize)> = localiser_chiffre(file);

        // println!("{pos_nb:?}");
        let hmap = check_cell(bomb, pos_nb);

        // println!("{hmap:?}");
        let su: usize = hmap.values().sum::<usize>();
        println!("final: {:?}", su);
        assert_eq!(su, 413)
    }
}

use std::{fs, collections::HashSet};

fn main() {
    let symbol = ['$','*','+','-','=','%','@','/','#'];
    let mut bomb = vec![];
    
    let file = fs::read_to_string("input").unwrap();
    for (index, line) in file.lines().enumerate() {
        for &sy in symbol.iter() {
            if line.contains(sy) && index > 0 {
                let pos = line.find(sy).unwrap();
                print!("{sy} at {index}:{pos}  |");
                bomb.push((index as i32, pos as i32));
            }
        }
    }
    
    let mut pos_nb = vec![];

    for (line_count, line) in file.lines().enumerate() {
        let mut number = String::new();

        for (j, l) in line.chars().enumerate() {
            if l.is_ascii_digit() {
                number = format!("{number}{l}");
            }
            if !l.is_ascii_digit() && !number.is_empty() {
                let nb = number.parse::<usize>().unwrap();
                pos_nb.push((line_count as i32, (j - number.len()) as i32, j as i32, nb));
                number = String::new();
            }
        }
    }
    println!("{pos_nb:?}");
    let mut jspfrr: HashSet<usize> = HashSet::new();
    for (ligne_signe, debut_signe) in bomb {
        for (ligne_chiffre, debut_chiffre, fin_chiffre, leditchiffre) in pos_nb.iter() {
            if ligne_signe == *ligne_chiffre {
                if debut_signe == debut_chiffre - 1 {
                    jspfrr.insert(leditchiffre.to_owned());
                }
                if debut_signe == *fin_chiffre {
                    jspfrr.insert(leditchiffre.to_owned());
                }
            }
            if ligne_signe - 1 == *ligne_chiffre && debut_signe >= *debut_chiffre - 1 && debut_signe <= *fin_chiffre + 1 {
                jspfrr.insert(leditchiffre.to_owned());
            }
            if ligne_signe + 1 == *ligne_chiffre && debut_signe >= *debut_chiffre - 1 && debut_signe <= *fin_chiffre + 1 {
                jspfrr.insert(leditchiffre.to_owned());
            }
        }
    }
    let su: usize = jspfrr.iter().sum();
    println!("{:?}", su);

}

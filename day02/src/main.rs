use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// pt 1
fn count_chars_list(v: Vec<&str>) -> (u32, u32) {
    v.iter()
        .map(|n| n.chars())
        .map(|chars| {
            let mut map = HashMap::new();
            chars.into_iter().for_each(|ch| {
                let counter = map.entry(ch).or_insert(0);
                *counter += 1;
            });
            // map char: count_of_char
            map
        })
        .map(|char_map| {
            let two_times = char_map.iter()
                .any(|(_, v)| v == &2);
            let three_times = char_map.iter()
                .any(|(_, v)| v == &3);
            // ( at least one letter that appears at least 2 times, at least one letter that appears 3 times)
            (two_times, three_times)
        })
        .fold((0, 0), |mut acc, n| {
            // iterates over list and counts how often each type was true
            match n {
                (true, true) => {
                    acc.0 += 1;
                    acc.1 += 1;
                    acc
                },
                (true, false) => {
                    acc.0 += 1;
                    acc
                },
                (false, true) => {
                    acc.1 += 1;
                    acc
                },
                _ => acc
            }
        })
}
#[test]
fn count_chars_list_test() {
    let test_data = vec!["abcdef", "bababc", "abbcde", "abcccd",
                         "aabcdd", "abcdee", "ababab"];
    assert_eq!(count_chars_list(test_data), (4, 3));
}

fn checksum(tup:(u32, u32)) -> u32 {
    let (two, three) = tup;
    two * three
}
 #[test]
fn checksum_test() {
    assert_eq!(checksum((4, 3)), 12);
}

// pt 2
fn off_by_one_id(v: Vec<&str>) -> Vec<&str> {
    let mut one_offs = vec![];
    for i in v.iter() {
        for j in v.iter() {
            let mut offs = 0;
            let j_vec = j.chars().into_iter().collect::<Vec<char>>();
            for (i_i, i_ch) in i.chars().into_iter().enumerate() {
                if i_ch != *j_vec.get(i_i).unwrap_or(&'_') {
                    offs += 1;
                }
            }
            if offs == 1 {
                one_offs.push(*j);
            }
        }
    }
    one_offs
}
#[test]
fn off_by_one_id_test() {
    let test_data = vec!["abcde", "fghij", "klmno", "pqrst",
                         "fguij", "axcye", "wvxyz"];
    assert_eq!(off_by_one_id(test_data), vec!["fguij", "fghij"]);
}

fn common_letters(v: Vec<&str>) -> String {
    let mut chopped = v[1].chars().into_iter().collect::<Vec<char>>();
    for (i, v) in  v[0].chars().into_iter().enumerate() {
        if v != chopped[i] {
            chopped.remove(i);
            break;
        }
    }
    chopped.into_iter().collect::<String>()
}
#[test]
fn common_letters_test() {
    assert_eq!(common_letters(vec!["fguij", "fghij"]), "fgij");
}

fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read file");
    let list_ids = content.split("\n").collect::<Vec<&str>>();

    let list_tuples = count_chars_list(list_ids.clone());
    let check = checksum(list_tuples);
    println!("pt 1: Checksum of list is {:?}", check);

    let common = common_letters(off_by_one_id(list_ids));
    println!("pt 2: Common letters by correct IDs are {:?}", common);
}

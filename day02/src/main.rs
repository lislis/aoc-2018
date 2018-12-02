use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read file");
    let list_ids = content.split("\n").collect::<Vec<&str>>();
    let list_tuples = count_chars_list(list_ids);
    let check = checksum(list_tuples);
    println!("Checksum of list is: {:?}", check);
}

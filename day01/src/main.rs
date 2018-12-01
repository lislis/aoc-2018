use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
    let freqs = content.split("\n").collect::<Vec<&str>>();
    let list_freq_changes = freqs.iter()
        .map(|n| n.replace("+", ""))
        .map(|n| n.trim().parse::<i64>())
        .collect::<Vec<Result<i64, _>>>();

    // pt 1
    let end_freq:i64 = list_freq_changes.iter()
        .fold(0, |acc, n| {
            match n {
                Ok(num) => acc + num,
                Err(error) => {
                    println!("error {}", error);
                    acc + 0
                }
            }
        });

    println!("pt 1, sum of all freqs: {:?}", end_freq);

    // pt2
    let mut existing_freqs = HashMap::new();
    let mut reached_twice: i64 = 0;
    let mut switch = false;
    let mut acc: i64 = 0;

    while !switch {
        for x in list_freq_changes.iter() {
            match x {
                Ok(val) => {
                    acc = acc + val;
                    if existing_freqs.contains_key(&acc) && switch == false {
                        reached_twice = acc.clone();
                        switch = true;
                    } else {
                        existing_freqs.insert(acc, 0);
                    }
                },
                _ => {}
            }
        }
    }
    println!("pt 2, first freq reached twice: {:?}", reached_twice);
}

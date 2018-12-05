use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut polymer = String::new();
    file.read_to_string(&mut polymer).expect("cannot read");


    //let text_mer = "dabAcCaCBAcCcaDA";

    let mut text_vec = polymer.chars().into_iter().collect::<Vec<char>>();
    let mut matches = false;

    loop {
        let _reacted_vec = polymer.chars().into_iter().enumerate()
            .map(|(i, ch)| {
                if ch == '\n' {
                    text_vec.remove(i);
                } else {
                    if text_vec.len() > i+1 {
                        let current = text_vec[i];
                        let next = text_vec[i+1];

                        let current_ch = current.to_string().to_lowercase();
                        let next_ch = next.to_string().to_lowercase();

                        if  current_ch == next_ch  {
                            match current.is_lowercase() {
                                true => {
                                    if next.is_uppercase() {
                                        text_vec.remove(i+1);
                                        text_vec.remove(i);
                                        matches = true
                                    }
                                },
                                false => {
                                    if next.is_lowercase() {
                                        text_vec.remove(i+1);
                                        text_vec.remove(i);
                                        matches = true
                                    }
                                }
                            }
                        }
                    }
                }
                ch
            })
            .collect::<Vec<char>>();

        if matches == false {
            break;
        } else {
            matches = false;
        }
    }

    // dec by one because the last element is a \n
    println!("{:?}", text_vec.len());
}

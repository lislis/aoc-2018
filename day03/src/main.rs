extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

type Claim = (u32, u32, u32, u32, u32);
type ClaimArea = Vec<(u32, u32)>;

fn parse_claim(st: &str) -> Claim {
    let r = st.split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
        .filter_map(|c| c.parse::<u32>().ok()).collect::<Vec<u32>>();

    if (r.len() == 5) { // weirdness
        (0, r[1], r[2], r[3], r[4])
    } else {
        (0,0,0,0,0)
    }
}
#[test]
fn parse_claim_test() {
    assert_eq!(parse_claim(&"#1 @ 1,3: 4x4"), (1, 1, 3, 4, 4));
    assert_eq!(parse_claim(&"#2 @ 3,1: 4x4"), (2, 3, 1, 4, 4));
    assert_eq!(parse_claim(&"#3 @ 5,5: 2x2"), (3, 5, 5, 2, 2));
}

fn claim_coord_vec(c: Claim) -> ClaimArea {
    let (_id, x, y, w, h) = c;

    let mut vec_coord = vec![];
    for y_val in y..(y+h) {
        for x_val in x..(x+w) {
            vec_coord.push((x_val, y_val));
        }
    }
    vec_coord
}
#[test]
fn claim_coord_vec_test() {
    let outcome = vec![(1, 3), (2, 3), (3, 3), (4, 3),
                       (1, 4), (2, 4), (3, 4), (4, 4),
                       (1, 5), (2, 5), (3, 5), (4, 5),
                       (1, 6), (2, 6), (3, 6), (4, 6)];
    assert_eq!(claim_coord_vec((1, 1, 3, 4, 4)), outcome);
}

fn all_the_coords(v:Vec<&str>) -> ClaimArea {
    v.iter()
        .map(|s| parse_claim(s))
        .map(|coord| claim_coord_vec(coord))
        .flatten()
        .collect()
}
#[test]
fn all_the_coords_test() {
    let data = vec!["#1 @ 1,3: 4x4",
                    "#2 @ 3,1: 4x4",
                    "#3 @ 5,5: 2x2"];
    assert_eq!(all_the_coords(data).len(), 36);
}

fn claimed_by_more_than_two(v: ClaimArea) -> u32 {
    let mut coord_map = HashMap::new();

    for x in v.iter() {
        let counter = coord_map.entry(x).or_insert(0);
        *counter += 1;
    }
    coord_map.into_iter()
        .filter(|&(_, v)| v > 1)
        .fold(0, |acc, (_,_)| acc + 1)
}
#[test]
fn claimed_by_more_than_two_test() {
    let data = all_the_coords(vec!["#1 @ 1,3: 4x4",
                                   "#2 @ 3,1: 4x4",
                                   "#3 @ 5,5: 2x2"]);
    assert_eq!(claimed_by_more_than_two(data), 4);
}


fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
    let claims = content.split("\n").collect::<Vec<&str>>();

    println!("pt 1: sum off all squares claimed at least twice: {:?}", claimed_by_more_than_two(all_the_coords(claims)));
}

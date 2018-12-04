use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Claim = (u32, u32, u32, u32, u32);
type ClaimArea = Vec<(u32, u32)>;
type ClaimAreaId = (u32, ClaimArea);

fn parse_claim(st: &str) -> Claim {
    let r = st.split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
        .filter_map(|c| c.parse::<u32>().ok()).collect::<Vec<u32>>();

    if r.len() == 5 { // weirdness
        (r[0], r[1], r[2], r[3], r[4])
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

fn claim_coord_with_id(c: Claim) -> ClaimAreaId {
    let (id, x, y, w, h) = c;

    let mut vec_coord = vec![];
    for y_val in y..(y+h) {
        for x_val in x..(x+w) {
            vec_coord.push((x_val, y_val));
        }
    }
    (id, vec_coord)
}
#[test]
fn claim_coord_with_id_test() {
    let outcome = (1, vec![(1, 3), (2, 3), (3, 3), (4, 3),
                       (1, 4), (2, 4), (3, 4), (4, 4),
                       (1, 5), (2, 5), (3, 5), (4, 5),
                       (1, 6), (2, 6), (3, 6), (4, 6)]);
    assert_eq!(claim_coord_with_id((1, 1, 3, 4, 4)), outcome);
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

fn all_the_coords_with_id(v: Vec<&str>) -> Vec<ClaimAreaId> {
    v.iter()
        .map(|s| parse_claim(s))
        .map(|coord| claim_coord_with_id(coord))
        .collect()
}
#[test]
fn all_the_coords_with_id_test() {
    let data = vec!["#1 @ 1,3: 4x4",
                    "#2 @ 3,1: 4x4",
                    "#3 @ 5,5: 2x2"];
    assert_eq!(all_the_coords_with_id(data).len(), 3);
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

fn claim_without_overlap(v: Vec<ClaimAreaId>) -> u32 {
    let mut all: HashSet<(u32, ClaimArea)> = HashSet::new();
    let mut counter_map = HashMap::new();
    let mut overlap_id: u32 = 0;

    v.iter()
        .for_each(|(i, vec)| {
            all.insert((*i, vec.to_vec()));
            vec.into_iter()
                .for_each(|j| {
                    let counter = counter_map.entry(j).or_insert(0);
                    *counter += 1;
                });
        });

    all.iter()
        .for_each(|(id, vec)| {
            let mut overlap = 0;
            vec.iter()
                .for_each(|coord| {
                    if counter_map.get(coord).unwrap() > &1 {
                        overlap += 1;
                    }
                });
            if overlap == 0 {
                overlap_id = *id;
            }
        });

    overlap_id
}


fn main() {
    let mut file = File::open("input.txt").expect("No file found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
    let claims = content.split("\n").collect::<Vec<&str>>();

    //println!("pt 1: sum off all squares claimed at least twice: {:?}", claimed_by_more_than_two(all_the_coords(claims)));

    let claim_id = claim_without_overlap(all_the_coords_with_id(claims));
    println!("pt 2: ID of the Claim without any overlap {:?}", claim_id);
}

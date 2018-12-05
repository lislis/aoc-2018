extern crate regex;

//use std::fs::File;
//use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


type GuardId = u32;

struct Shift {
    guard_id: u32,
    day: String,
    minutes_asleep: HashSet<u32>
}


fn main() {
    let test_data = vec!["[1518-11-01 00:00] Guard #10 begins shift",
                         "[1518-11-01 00:05] falls asleep",
                         "[1518-11-01 23:58] Guard #99 begins shift",
                         "[1518-11-01 00:25] wakes up",
                         "[1518-11-01 00:30] falls asleep",
                         "[1518-11-01 00:55] wakes up",
                         "[1518-11-05 00:03] Guard #99 begins shift",
                         "[1518-11-02 00:40] falls asleep",
                         "[1518-11-02 00:50] wakes up",
                         "[1518-11-03 00:05] Guard #10 begins shift",
                         "[1518-11-03 00:24] falls asleep",
                         "[1518-11-03 00:29] wakes up",
                         "[1518-11-04 00:02] Guard #99 begins shift",
                         "[1518-11-04 00:36] falls asleep",
                         "[1518-11-04 00:46] wakes up",
                         "[1518-11-05 00:45] falls asleep",
                         "[1518-11-05 00:55] wakes up"];

    // tells us which days which guard is on duty
    let mut days: HashMap<&str, GuardId> = HashMap::new();
    let mut sleep_marker: Vec<(&str, u32)> = vec![];
    let mut awake_marker: Vec<(&str, u32)> = vec![];

    let re = Regex::new(r"[:\[\] ]+").expect("wrong regex");

    let mut parsed = test_data.into_iter()
        .map(|string| re.split(string).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    parsed.sort();

    parsed.into_iter()
        .for_each(|vec| {
            if vec[4] == "Guard" {
                let id = vec[5].replace("#", "").parse::<u32>().ok();
                // be aware that some guards start before midnight
                // check if hour == 23
                // then count up day +1
                days.insert(vec[1], id.unwrap_or(0));
            }
            if vec[4] == "falls" {
                let min = vec[3].parse::<u32>().ok();
                sleep_marker.push((vec[1], min.unwrap()));
            }
            if vec[4] == "wakes" {
                let min = vec[3].parse::<u32>().ok();
                awake_marker.push((vec[1], min.unwrap()));
            }
        });

    sleep_marker.sort();
    awake_marker.sort();

    days.into_iter().map(|(day, guard)| {
        let sleep = sleep_marker.iter()
            .filter(|(d, min)| d == &day )
            .fold(vec![], |mut acc, (d, min)| {
                acc.push(min);
                acc
            });
        println!("{:?}", sleep);

        let awake = awake_marker.iter()
            .filter(|(d, min)| d == &day )
            .fold(vec![], |mut acc, (d, min)| {
                acc.push(min);
                acc
            });
        println!("{:?}", awake);

        // range from sleep to awake,
        // return tuple day, vector of  minute asleep
         2
    }).collect::<Vec<u32>>();


    //println!("{:?}", sleep_marker);
    //println!("{:?}", foo);
    //   .map(|vec| {
     //       println!("{:?}", vec);
     //       2
     //   })
     //   .collect::<Vec<u32>>();

    //let mut shifts = HashMap::new();
    // key: day




}

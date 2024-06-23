#![crate_name = "2852"]
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<String>);
    let mut score_1 = 0;
    let mut score_2 = 0;
    let mut lead_1 = 0;
    let mut lead_2 = 0;
    let mut lead = 0;
    let mut previous_event_time = 0;
    for _ in 0..n {
        let team = input.next().unwrap().parse::<usize>().unwrap();
        let time = input.next().unwrap();
        let mm = time[0..2].parse::<usize>().unwrap();
        let ss = time[3..5].parse::<usize>().unwrap();
        let time = mm * 60 + ss;
        match lead {
            1 => {
                lead_1 += time - previous_event_time;
            }
            2 => {
                lead_2 += time - previous_event_time;
            }
            _ => lead = team,
        }
        match team {
            1 => score_1 += 1,
            2 => score_2 += 1,
            _ => {}
        }
        if score_1 == score_2 {
            lead = 0;
        }
        previous_event_time = time;
    }
    match lead {
        1 => {
            lead_1 += 48 * 60 - previous_event_time;
        }
        2 => {
            lead_2 += 48 * 60 - previous_event_time;
        }
        _ => {}
    }
    println!("{:02}:{:02}", lead_1 / 60, lead_1 % 60);
    println!("{:02}:{:02}", lead_2 / 60, lead_2 % 60);
}
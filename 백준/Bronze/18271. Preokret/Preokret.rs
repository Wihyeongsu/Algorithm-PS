use std::{
    io::{self, stdin, Read},
    vec,
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut turnover = 0;
    let mut successive_goals = 0;
    let mut tie = 1;
    let mut city = 0;
    let mut oppo = 0;
    let mut pre_score = 0;
    for _ in 0..n {
        let mut who_scores = input.next().unwrap();
        match who_scores {
            1 => {
                city += 1;
            }
            2 => {
                oppo += 1;
            }
            _ => {}
        }
        if city == oppo {
            tie += 1;
        }
        if who_scores == pre_score {
            successive_goals += 1;
            let mut who_scored = {
                match who_scores {
                    1 => {
                        who_scores = city;
                        oppo
                    }
                    2 => {
                        who_scores = oppo;
                        city
                    }
                    _ => 0,
                }
            };
            if who_scores > who_scored && who_scores - successive_goals < who_scored {
                turnover = turnover.max(successive_goals);
            }
        } else {
            pre_score = who_scores;
            successive_goals = 1;
        }
    }
    println!("{city} {oppo}");
    println!("{tie}");
    println!("{turnover}");
}

use std::{
    io::{stdin, Read},
    ops::RemAssign,
    vec,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut num: Vec<usize> = input.collect();
    let mut remainder: Vec<usize> = vec![0; n + 1];
    let mut remainder_cnt = vec![0; m];
    num.insert(0, 0);
    for i in 1..=n {
        num[i] += num[i - 1];
        remainder[i] = num[i] % m;
        remainder_cnt[remainder[i]] += 1;
    }
    let mut answer: usize = 0;
    answer += remainder_cnt[0];
    for cnt in remainder_cnt {
        if cnt < 2 {
            continue;
        }
        answer += cnt * (cnt - 1) / 2;
    }

    println!("{answer}");
    // println!("{:#?}", remainder);
}
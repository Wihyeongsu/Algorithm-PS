use std::{
    io::{stdin, Read},
    vec,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let s = input.next().unwrap();
    let mut num: Vec<usize> = input.collect();
    num.insert(0, 0);
    for i in 1..=n {
        num[i] += num[i - 1];
    }
    let mut answer = std::usize::MAX;
    let mut start = 0;
    let mut end = 1;
    while start < end && end <= n {
        match num[end] - num[start] >= s {
            true => {
                answer = answer.min(end - start);
                // println!("={answer}={}=", num[end] - num[start]);
                start += 1;
            }
            false => end += 1,
        }
    }
    if answer == std::usize::MAX {
        answer = 0;
    }
    println!("{answer}");
    // println!("{:#?}", num);
}

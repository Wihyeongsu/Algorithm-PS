use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);
    let n = input.next().unwrap();
    let house: Vec<isize> = input.collect();
    let mut answer = std::usize::MAX;
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            let mut sum = 0;
            for k in 0..n as usize {
                sum += match (house[k] - house[i]).pow(2) > (house[k] - house[j]).pow(2) {
                    true => (house[k] - house[j]).pow(2),
                    false => (house[k] - house[i]).pow(2),
                }
            }
            answer = answer.min(sum as usize);
        }
    }
    print!("{answer}");
}
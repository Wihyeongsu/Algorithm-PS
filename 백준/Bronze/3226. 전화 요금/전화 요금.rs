use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<String>);
    let mut answer = 0;

    for _ in 0..n  {
        let time = input.next().unwrap();
        let mut time = time.split(":");
        let hh = time.next().unwrap().parse::<usize>().unwrap();
        let mm = time.next().unwrap().parse::<usize>().unwrap();
        let dd = input.next().unwrap().parse::<usize>().unwrap();

        let time = hh*60 + mm;
        if time <= 7*60 && 7*60 <= time + dd {
            answer += 5*(7*60-time) + 10*(time+dd-7*60);
        }
        else if time <= 19*60 && 19*60 <= time + dd {
            answer += 10*(19*60-time) + 5*(time+dd-19*60);
        }
        else if 7*60 <= time && time + dd <= 19*60 {
            answer += 10*dd;
        }
        else {
            answer += 5*dd;
        }
    }
    print!("{answer}");
}
use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();
    println!("{}", a.parse::<isize>().unwrap() + b.parse::<isize>().unwrap() - c.parse::<isize>().unwrap());
    println!("{}", (a.to_owned() + b).parse::<isize>().unwrap() - c.parse::<isize>().unwrap());
}
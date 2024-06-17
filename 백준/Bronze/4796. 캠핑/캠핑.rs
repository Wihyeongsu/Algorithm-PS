use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    stdin().read_to_string(&mut input);
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);
    let mut i = 0;
    loop {
        let l = input.next().unwrap();
        let p = input.next().unwrap();
        let v = input.next().unwrap();
        if l == p && p == v && v == 0 {break;}
        i += 1;
        println!("Case {i}: {}", l*(v/p) + std::cmp::min(l,v%p));
    }
}
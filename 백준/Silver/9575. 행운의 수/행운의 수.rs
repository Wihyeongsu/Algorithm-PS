use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn is_fortune_number(mut num: usize) -> bool {
    while num > 0 {
        let fortune_num = num % 10;
        num /= 10;
        match fortune_num {
            5 | 8 => {}
            _ => {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let t = input.next().unwrap();
    for _ in 0..t {
        let size_a = input.next().unwrap();
        let a: Vec<usize> = input.by_ref().take(size_a).collect();

        let size_b = input.next().unwrap();
        let b: Vec<usize> = input.by_ref().take(size_b).collect();

        let size_c = input.next().unwrap();
        let c: Vec<usize> = input.by_ref().take(size_c).collect();

        let mut set = HashSet::new();
        for i in &a {
            for j in &b {
                for k in &c {
                    let num = i + j + k;
                    if is_fortune_number(num) {
                        set.insert(num);
                    }
                }
            }
        }
        println!("{}", set.len());
    }
}
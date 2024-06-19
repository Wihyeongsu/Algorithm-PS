use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_ascii_whitespace();
    let mut students: HashMap<String, usize> = HashMap::new();
    // set HashMap
    for _ in 0..n {
        let name = input.next().unwrap().parse::<String>().unwrap();
        students.insert(name, 0);
    }

    // set popularity
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<String>);
    for name in input {
        students.entry(name).and_modify(|value| *value += 1);
    }
    // sort
    let mut sorted_students: Vec<(String, usize)> = students.into_iter().collect();
    sorted_students.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for i in sorted_students.iter() {
        println!("{} {}", i.0, i.1);
    }
}

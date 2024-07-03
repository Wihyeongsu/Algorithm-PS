use std::{
    io::{stdin, Read},
    vec,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut idx = 1;
    // set
    while let Some(h) = input.next() {
        let mut answer = 0;
        // check
        while let Some((h_in, idx_in)) = stack.pop() {
            if h_in > h {
                stack.push((h_in, idx_in));
                answer = idx_in;
                break;
            }
        }
        stack.push((h, idx));
        idx += 1;
        print!("{answer} ");
    }
    // println!("{:?}", stack);
}

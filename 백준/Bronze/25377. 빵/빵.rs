use std::{
    io::{self, stdin, Read},
    vec,
};

fn combination(
    sum_x: isize,
    sum_y: isize,
    mut sum_end_x: isize,
    mut sum_end_y: isize,
    mut min: isize,
    depth: usize,
    idx: usize,
    cord: &Vec<(isize, isize)>,
) -> isize {
    if depth <= 0 {
        let sum_start_x = sum_x - sum_end_x;
        let sum_start_y = sum_y - sum_end_y;
        return min.min((sum_end_x - sum_start_x).pow(2) + (sum_end_y - sum_start_y).pow(2));
    }
    for i in idx..cord.len() {
        sum_end_x += cord[i].0;
        sum_end_y += cord[i].1;
        min = combination(
            sum_x,
            sum_y,
            sum_end_x,
            sum_end_y,
            min,
            depth - 1,
            i + 1,
            &cord,
        );
        sum_end_x -= cord[i].0;
        sum_end_y -= cord[i].1;
    }
    min
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut min: isize = 1001;
    for _ in 0..n {
        let a = input.next().unwrap();
        let b = input.next().unwrap();

        if a <= b {
            min = min.min(b as isize);
        }
    }
    if min == 1001 {
        min = -1;
    }
    print!("{min}");
}

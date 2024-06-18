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
    let t = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    for _ in 0..t {
        let mut min: isize = 80000000000;
        let n = input.next().unwrap().parse::<usize>().unwrap();
        // set point
        let mut cord: Vec<(isize, isize)> = vec![(0, 0); n];
        let mut sum_x = 0;
        let mut sum_y = 0;
        for i in 0..n {
            let x = input.next().unwrap().parse::<isize>().unwrap();
            let y = input.next().unwrap().parse::<isize>().unwrap();
            cord[i] = (x, y);
            sum_x += x;
            sum_y += y;
        }
        // sum vector
        let min = combination(sum_x, sum_y, 0, 0, min, n / 2, 0, &cord) as f64;
        println!("{}", min.sqrt());
    }
}
use std::{
    io::{stdin, Read},
    vec,
};

// 우측에서부터 탐색하여 스택에 저장
// 스택에 있는 값과 비교하여 스택에 있는 값보다 작을 경우 삽입
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: Vec<usize> = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>)
        .collect();
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    let mut output: Vec<isize> = Vec::with_capacity(n);
    while let Some(token) = input.pop() {
        loop {
            match stack.pop() {
                Some(num) if num > token => {
                    stack.push(num);
                    output.push(num as isize);
                    stack.push(token);
                    break;
                }
                Some(_) => {}
                None => {
                    stack.push(token);
                    output.push(-1);
                    break;
                }
            }
        }
    }
    while let Some(answer) = output.pop() {
        print!("{answer} ");
    }
}

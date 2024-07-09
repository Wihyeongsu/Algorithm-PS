use std::{
    io::{stdin, Read},
    vec,
};

// 언덕을 하나 넘어갈 때마다 스택 영역의 크기를 계산

fn find(stack: &mut Vec<(usize, usize)>, token: usize, index: usize) -> usize {
    let mut area = if let Some(&(_, h)) = stack.last() {
        h
    } else {
        0
    };
    while let Some((idx, height)) = stack.pop() {
        if height < token {
            stack.push((idx, height));
            break;
        }
        let width = if stack.is_empty() {
            index
        } else {
            index - 1 - stack.last().unwrap().0
        };
        area = area.max(height * width);

        // println!("{index} {idx}");
        // println!("{height} * {width} = {area}");
    }
    area
}
fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input
            .trim()
            .split_ascii_whitespace()
            .flat_map(str::parse::<usize>);
        let n = input.next().unwrap();
        let mut input: Vec<(usize, usize)> = input.enumerate().collect();
        if n == 0 {
            break;
        }
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(n);
        let mut answer = 0;

        while let Some((rindex, token)) = input.pop() {
            let index = n - 1 - rindex;
            // println! {"input {index} {token}"};
            // find
            answer = answer.max(find(&mut stack, token, index));
            stack.push((index, token));
        }
        answer = answer.max(find(&mut stack, 0, n));

        println!("{answer}");
    }
}

use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    let mut a = 0;
    let mut b = 0;
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<String>);

    loop {
        let i: usize = input.next().unwrap().parse().unwrap();
        match i {
            1 => {
                if input.next().unwrap() == "A" {
                    a = input.next().unwrap().parse().unwrap();
                }
                else {
                    b = input.next().unwrap().parse().unwrap();
                }
            }
            2 => {
                if input.next().unwrap() == "A" {
                    println!("{}", a);
                }
                else {
                    println!("{}", b);
                }
            },
            3..=6 => {
                let x = input.next().unwrap();
                let y = input.next().unwrap();
                let a_val = a;
                let b_val = b;
                let z = {
                    if x == "A" {&mut a}
                    else {&mut b}
                };
                let x = {
                    if x == "A" {a_val}
                    else {b_val}
                };
                let y = {
                    if y == "A" {a_val}
                    else {b_val}
                };
                *z = {
                    match i {
                        3 => x + y,
                        4 => x * y,
                        5 => x - y,
                        6 => x / y,
                        _ => 0,
                    }
                };

            }
            7 => break,
            _ => {},
        }

    }

}
use std::io::{stdin, Read};

fn is_prime(num: usize) -> bool {
    match num {
        0 | 1 => return false,
        _ => {
            for n in 2..=(num as f64).sqrt() as usize {
                if num % n == 0 {
                    return false;
                }
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
        let n = input.next().unwrap();
        for num in n.. {
            if is_prime(num) {
                println!("{num}");
                break;
            }
        }
    }
}
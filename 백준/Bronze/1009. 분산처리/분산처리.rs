use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut buffer = buffer.split_ascii_whitespace().flat_map(str::parse::<usize>);
    
    for _ in 0..t {
        let mut arr = [0; 5];
        let a = buffer.next().unwrap() % 10;
        let b = buffer.next().unwrap();
        if a == 0 {
            println!("10");
        }
        else{
            let mut answer = a;
            arr[0] = a;
            for i in 1..b {
                arr[i] = (arr[i - 1] * a) % 10;
                answer = arr[i];
                if arr[0] == arr[i] {
                    answer = arr[(b - 1) % i];
                    break;
                }
            }
            println!("{}", answer);
        }
    }
}
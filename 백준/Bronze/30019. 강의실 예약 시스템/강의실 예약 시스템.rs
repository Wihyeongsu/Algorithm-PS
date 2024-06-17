use std::io::{stdin, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut vec = vec![1; n + 1];

    let stdin = stdin();
    let mut lines = BufReader::new(stdin.lock()).lines();

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();

        let k = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();
        let s = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();
        let e = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();

        if vec[k] <= s {
            vec[k] = e;
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
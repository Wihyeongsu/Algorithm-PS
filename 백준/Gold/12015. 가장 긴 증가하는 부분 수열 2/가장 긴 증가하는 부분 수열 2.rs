use std::{
    io::{self, Read},
    str::SplitAsciiWhitespace,
    vec,
};

#[derive(Debug, Default, Clone)]
struct LISBinarySearch {
    sequence: Vec<isize>,
    tails: Vec<isize>,
    position: Vec<usize>,
    prev: Vec<Option<usize>>,
    lis: Vec<isize>,
}

impl LISBinarySearch {
    fn new(sequence: Vec<isize>) -> Self {
        let n = sequence.len();

        let mut lis = LISBinarySearch {
            sequence,
            tails: Vec::with_capacity(n),
            position: Vec::with_capacity(n),
            prev: vec![None; n],
            lis: Vec::new(),
        };

        lis.compute_lis();
        lis
    }

    fn compute_lis(&mut self) {
        let mut len = 1;
        self.tails.push(self.sequence[0]);
        self.position.push(0);

        for i in 1..self.sequence.len() {
            let token = self.sequence[i];
            // Binary Search
            if let Some(&num) = self.tails.last() {
                if num < token {
                    self.tails.push(token);
                    self.position.push(i);
                    len += 1;
                    self.prev[i] = Some(self.position[len - 2]);
                } else if num >= token {
                    let index = self.tails.partition_point(|&x| x < token);
                    self.tails[index] = token;
                    self.position[index] = i;
                    if index == 0 {
                        self.prev[index] = None;
                    } else {
                        self.prev[i] = Some(self.position[index - 1]);
                    }
                }
            }
        }
        self.set_lis();
    }

    fn set_lis(&mut self) {
        let last_index = self.position.len() - 1;
        let mut position = self.position[last_index];
        self.lis.push(self.sequence[position]);
        while let Some(lis_position) = self.prev[position] {
            self.lis.push(self.sequence[lis_position]);
            position = lis_position;
        }
        self.lis.reverse();
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace();
    let input_unwrap = |x: &mut SplitAsciiWhitespace| x.next().unwrap().parse::<usize>().unwrap();

    let n = input_unwrap(&mut input);
    let sequence: Vec<isize> = input.flat_map(str::parse::<isize>).collect();
    let lis = LISBinarySearch::new(sequence);

    // println!("{lis:#?}");
    println!("{}", lis.lis.len());

    Ok(())
}

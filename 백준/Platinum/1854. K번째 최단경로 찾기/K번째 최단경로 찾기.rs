use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
        io::{self, Read},
    str::SplitAsciiWhitespace,
};

pub fn solve(
    n: usize,
    m: usize,
    k: usize,
    roads: &Vec<(usize, usize, usize)>,
) -> Vec<isize> {
    let mut graph = vec![Vec::new(); n + 1];
    for &(a, b, c) in roads {
        graph[a].push((b, c));
    }

    let mut distances = vec![VecDeque::new(); n + 1];
    let mut pq = BinaryHeap::new();

    distances[1].push_back(0);
    pq.push((Reverse(0), 1));

    while let Some((Reverse(dist), node)) = pq.pop() {
        if distances[node].len() == k && dist > distances[node][k - 1] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let next_dist = dist + weight;
            if distances[next].len() < k {
                distances[next].push_back(next_dist);
                distances[next].make_contiguous().sort_unstable();
                pq.push((Reverse(next_dist), next));
            } else if next_dist < distances[next][k - 1] {
                distances[next].pop_back();
                distances[next].push_back(next_dist);
                distances[next].make_contiguous().sort_unstable();
                pq.push((Reverse(next_dist), next));
            }
        }
    }

    
    distances
    .iter()
    .skip(1)
    .map(|d| if d.len() < k { -1 } else { d[k - 1] as isize })
    .collect()
        
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace();
    let input_tuple = |x: &mut SplitAsciiWhitespace| {
        (
            x.next().unwrap().parse::<usize>().unwrap(),
            x.next().unwrap().parse::<usize>().unwrap(),
            x.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let (n, m, k) = input_tuple(&mut input);

    let mut paths: Vec<(usize, usize, usize)> = Vec::new();

    for _ in 0..m {
        let path = input_tuple(&mut input);

        paths.push(path)
    }

    let answer = solve(n, m, k, &paths);
    for &i in &answer {
        println!("{i}");
    }
    
    Ok(())
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::{self, BufRead},
};

const MAX_COST: usize = 100_000_000;

fn dijkstra(
    n: usize,
    s: usize,
    d: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    disabled: &Vec<Vec<bool>>,
) -> (usize, Vec<Vec<usize>>) {
    let mut pq = BinaryHeap::new();
    let mut dij = vec![MAX_COST; n];
    let mut from_node = vec![Vec::new(); n];
    pq.push(Reverse((0, s)));
    dij[s] = 0;

    while let Some(Reverse((distance, through))) = pq.pop() {
        if through == d {
            break;
        }

        if dij[through] < distance {
            continue;
        }

        for (i, &(to, cost)) in adj[through].iter().enumerate() {
            if disabled[through][i] {
                continue;
            }

            let next_cost = distance + cost;
            if dij[to] > next_cost {
                dij[to] = next_cost;
                from_node[to].clear();
                from_node[to].push(through);
                pq.push(Reverse((next_cost, to)));
            } else if dij[to] == next_cost {
                from_node[to].push(through);
            }
        }
    }

    (dij[d], from_node)
}

fn remove_shortest_paths(
    d: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    from_node: &Vec<Vec<usize>>,
    disabled: &mut Vec<Vec<bool>>,
) {
    let mut stack = vec![d];
    while let Some(to) = stack.pop() {
        for &from in &from_node[to] {
            for (i, &(next, _)) in adj[from].iter().enumerate() {
                if next == to && !disabled[from][i] {
                    disabled[from][i] = true;
                    stack.push(from);
                    break;
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        input.clear();
        stdin.read_line(&mut input)?;
        let mut iter = input.split_whitespace();
        let s: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut disabled: Vec<Vec<bool>> = vec![Vec::new(); n];
        for _ in 0..m {
            input.clear();
            stdin.read_line(&mut input)?;
            let mut iter = input.split_whitespace();
            let u: usize = iter.next().unwrap().parse().unwrap();
            let v: usize = iter.next().unwrap().parse().unwrap();
            let p: usize = iter.next().unwrap().parse().unwrap();
            adj[u].push((v, p));
            disabled[u].push(false);
        }

        let (min_cost, from_node) = dijkstra(n, s, d, &adj, &disabled);
        remove_shortest_paths(d, &adj, &from_node, &mut disabled);

        let (almost_min_cost, _) = dijkstra(n, s, d, &adj, &disabled);
        if almost_min_cost == MAX_COST {
            println!("-1");
        } else {
            println!("{}", almost_min_cost);
        }
    }

    Ok(())
}
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, Read},
    vec,
};

// Prim
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);
    let v = input.next().unwrap() as usize;
    let e = input.next().unwrap() as usize;
    let mut adj = vec![Vec::new(); v + 1];
    let mut visited = vec![false; v + 1];
    let mut pq = BinaryHeap::new();
    let mut answer = 0;
    pq.push(Reverse((0, 1)));
    for _ in 0..e {
        let n1 = input.next().unwrap() as usize;
        let n2 = input.next().unwrap() as usize;
        let w = input.next().unwrap();
        adj[n1].push((w, n2));
        adj[n2].push((w, n1));
    }
    while let Some(Reverse((w, to))) = pq.pop() {
        if visited[to] {
            continue;
        }
        answer += w;
        visited[to] = true;
        let mut visit_all = true;
        for i in 1..=v {
            if !visited[i] {
                visit_all = false;
                break;
            }
        }
        if visit_all == true {
            break;
        }
        for (weight, to_node) in &adj[to] {
            if visited[*to_node] {
                continue;
            }
            pq.push(Reverse((*weight, *to_node)));
        }
    }
    print!("{answer}");
}

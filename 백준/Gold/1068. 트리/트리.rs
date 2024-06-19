use std::io::{stdin, Read};

fn dfs(mut cnt: usize, node: usize, edge_down: &Vec<Vec<bool>>) -> usize {
    let mut is_leaf = true;
    for i in 0..edge_down[node].len() {
        if edge_down[node][i] {
            is_leaf = false;
            cnt = dfs(cnt, i, edge_down);
        }
    }
    match is_leaf {
        true => cnt += 1,
        false => {}
    };
    cnt
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<isize> = input
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<isize>)
        .collect();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let node_removed = input.trim().parse::<usize>().unwrap();
    let mut edge_down = vec![vec![false; n]; n];
    let mut root = 0;
    for node in 0..n {
        match vec[node] {
            -1 => root = node,
            _ => {
                edge_down[vec[node] as usize][node] = true;
            }
        }
    }
    for i in 0..n {
        edge_down[node_removed][i] = false;
        edge_down[i][node_removed] = false;
    }
    let answer;
    match node_removed {
        a if a == root => answer = 0,
        _ => answer = dfs(0, root, &edge_down),
    }
    print!("{answer}");
}

use std::{
    io::{stdin, Read},
    vec,
};

const INF: usize = 1000000;

fn get_min_node(n: usize, visited: &Vec<bool>, d: &Vec<usize>) -> usize {
    let mut idx = 0;
    let mut min = INF;
    for i in 1..=n {
        if !visited[i] && min > d[i] {
            min = d[i];
            idx = i;
        }
    }
    idx
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let x = input.next().unwrap();
    let mut adj = vec![vec![INF; n + 1]; n + 1];
    // set
    for _ in 0..m {
        let from = input.next().unwrap();
        let to = input.next().unwrap();
        let t = input.next().unwrap();
        adj[from][to] = t;
    }
    // dij
    let mut go = vec![INF; n + 1];
    let mut come = vec![INF; n + 1];
    let mut visited_go = vec![false; n + 1];
    let mut visited_come = vec![false; n + 1];
    visited_go[x] = true;
    visited_come[x] = true;
    for i in 1..=n {
        go[i] = adj[i][x];
        come[i] = adj[x][i];
    }
    for _ in 2..n {
        let current_go = get_min_node(n, &visited_go, &go); // go to x
        let current_come = get_min_node(n, &visited_come, &come); // back from x
        visited_go[current_go] = true;
        visited_come[current_come] = true;
        for i in 1..=n {
            if !visited_come[i] && come[i] > come[current_come] + adj[current_come][i] {
                come[i] = come[current_come] + adj[current_come][i];
            }
            if !visited_go[i] && go[i] > adj[i][current_go] + go[current_go] {
                go[i] = adj[i][current_go] + go[current_go];
            }
        }
    }
    // print
    let mut answer = 0;
    for i in 1..=n {
        if i == x {
            continue;
        }
        let go = go[i];
        let come = come[i];
        answer = answer.max(go + come);
    }
    print!("{answer}");
}
use std::{
    cmp::max,
    collections::{hash_map::Entry, HashMap},
    io::{stdin, Read},
    vec,
};

fn dfs(
    node: usize,
    mut sum: usize,
    mut visited: &mut Vec<bool>,
    adj: &Vec<HashMap<usize, usize>>,
) -> Option<usize> {
    // println!("Enter {node} {sum}");
    let mut ret: Option<usize> = None;
    let mut is_terminal = true;
    let mut all_visited = true;
    visited[node] = true;
    for i in &*visited {
        if !i {
            all_visited = false;
            break;
        }
    }
    for (to, d) in &adj[node] {
        if visited[*to] {
            continue;
        }
        is_terminal = false;
        if let Some(mut number) = dfs(*to, sum + *d, &mut visited, &adj) {
            if let Some(value) = ret {
                number = number.max(value);
            }
            ret = Some(number);
        }
    }
    if node == 0 && all_visited {
        // println!("Escape {node} {sum} {:?}", Some(sum));
        return Some(sum);
    } else if all_visited {
        if let Some(value) = adj[node].get(&0) {
            ret = dfs(0, sum + value, visited, adj)
        }
    }
    // println!("Escape {node} {sum} {:?}", ret);
    visited[node] = false;
    ret
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut adj: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n + 1];
    // set
    for _ in 0..m {
        let from = input.next().unwrap();
        let to = input.next().unwrap();
        let d = input.next().unwrap();
        if from == to {
            continue;
        }
        if let Entry::Occupied(mut entry) = adj[from].entry(to) {
            *entry.get_mut() = max(*entry.get(), d);
        } else {
            adj[from].insert(to, d);
        }
    }
    let mut visited = vec![false; n + 1];
    if let Some(answer) = dfs(0, 0, &mut visited, &adj) {
        println!("{answer}");
    } else {
        println!("-1");
    }
    // println!("{:#?}", adj);
}
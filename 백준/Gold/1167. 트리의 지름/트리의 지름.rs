use std::io::{stdin, Read};

fn dfs(
    root: usize,
    mut sum_w: usize,
    mut max: usize,
    mut depth_node: usize,
    visited: &mut Vec<bool>,
    adj_list: &Vec<Vec<(usize, usize)>>,
) -> (usize, usize) {
    visited[root] = true;
    let mut is_terminal_node = true;
    for (to, w) in &adj_list[root] {
        match visited[*to] {
            true => {}
            false => {
                let ret;
                is_terminal_node = false;
                (depth_node, ret) = dfs(*to, sum_w + w, max, depth_node, visited, adj_list);
                max = max.max(ret);
            }
        }
    }
    if is_terminal_node && max < sum_w {
        max = max.max(sum_w);
        depth_node = root;
    }
    (depth_node, max)
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);
    // set edge
    let mut adj_list: Vec<Vec<(usize, usize)>> = vec![Vec::new(); v + 1];
    let mut node = 0;
    for _ in 0..v {
        let mut linked_node_cnt = 0;
        let from_node = input.next().unwrap() as usize;
        let mut to_node = input.next().unwrap();
        while to_node != -1 {
            let w = input.next().unwrap();
            adj_list[from_node].push((to_node as usize, w as usize));
            linked_node_cnt += 1;
            to_node = input.next().unwrap();
        }
        if linked_node_cnt == 1 {
            node = from_node;
        }
    }
    let mut answer = 0;
    let mut visited = vec![false; v + 1];
    let root = dfs(node, 0, 0, node, &mut visited, &adj_list).0;
    visited = vec![false; v + 1];
    let distance = dfs(root, 0, 0, root, &mut visited, &adj_list).1;
    answer = answer.max(distance);
    print!("{answer}");
}

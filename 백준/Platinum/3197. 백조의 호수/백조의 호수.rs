use std::{
    io::{stdin, Read},
    sync::mpsc::TryRecvError,
    vec,
};
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    // match pattern 변수 문제
    // fn find(&mut self, x: usize) -> usize {
    //     self.parent[x] = match self.parent[x] {
    //         x => x,
    //         _ => self.find(self.parent[x]),
    //     };
    //     self.parent[x]
    // }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_x] = root_y;
            self.rank[root_y] += 1;
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>);
    let r = input.next().unwrap();
    let c = input.next().unwrap();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.replace('\n', "");
    let mut input = input.chars();
    let mut map = vec![vec!['.'; c]; r];
    let mut swan = Vec::with_capacity(2);
    let mut water = Vec::with_capacity(r * c);
    let mut set = UnionFind::new(r * c);
    for i in 0..r {
        for j in 0..c {
            map[i][j] = input.next().unwrap();
            if map[i][j] == 'L' {
                swan.push(c * i + j);
            }
            if !(map[i][j] == 'X') {
                water.push(c * i + j);
            }
        }
    }

    let mut days = 0;

    let mut visited = vec![vec![false; c]; r];
    let mut to_water = Vec::with_capacity(r * c);

    // setting
    while let Some(value) = water.pop() {
        let y = value / c;
        let x = value % c;
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        for i in 0..4 {
            let ny = y as isize + DY[i];
            let nx = x as isize + DX[i];
            if nx < 0 || c as isize <= nx || ny < 0 || r as isize <= ny {
                continue;
            }
            if visited[ny as usize][nx as usize] {
                continue;
            }
            if map[ny as usize][nx as usize] == 'X' {
                to_water.push(c * ny as usize + nx as usize);
            } else {
                set.union(value, c * ny as usize + nx as usize);
            }
        }
    }

    loop {
        match set.connected(swan[0], swan[1]) {
            true => {
                println!("{days}");
                break;
            }
            false => {
                days += 1;
                // melt
                while let Some(value) = to_water.pop() {
                    let y = value / c;
                    let x = value % c;

                    map[y][x] = '.';
                    water.push(value);

                    for i in 0..4 {
                        let ny = y as isize + DY[i];
                        let nx = x as isize + DX[i];
                        if nx < 0 || c as isize <= nx || ny < 0 || r as isize <= ny {
                            continue;
                        }
                    }
                }

                // union
                while let Some(value) = water.pop() {
                    let y = value / c;
                    let x = value % c;
                    if visited[y][x] {
                        continue;
                    }
                    visited[y][x] = true;
                    for i in 0..4 {
                        let ny = y as isize + DY[i];
                        let nx = x as isize + DX[i];
                        if nx < 0 || c as isize <= nx || ny < 0 || r as isize <= ny {
                            continue;
                        }
                        if map[ny as usize][nx as usize] == 'X' {
                            to_water.push(c * ny as usize + nx as usize);
                        } else {
                            set.union(value, c * ny as usize + nx as usize);
                        }
                    }
                }
            }
        }
    }

    // println!("{:#?}", map);
}

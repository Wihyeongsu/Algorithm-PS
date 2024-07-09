#![crate_name = "3197"]
use std::{
    io::{stdin, Read},
    sync::mpsc::TryRecvError,
    vec,
};

// unionfind를 이용하여 빙판으로 나눠진 물을 집합으로 관리
// {
// 집합을 형성하면서 녹을 빙판 조사
// 빙판 녹이기
// }
// 위 단계 반복
// 각 백조가 속한 물의 집합이 연결되면 정답

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

struct VisitedMask {
    masks: Vec<u64>,
    height: usize,
    width: usize,
}

impl VisitedMask {
    fn new(height: usize, width: usize) -> Self {
        let total_bits = width * height;
        let num_u64s = (total_bits + 63) / 64; // 올림 나눗셈
        VisitedMask {
            masks: vec![0; num_u64s],
            height,
            width,
        }
    }

    fn is_visited(&self, y: usize, x: usize) -> bool {
        if y >= self.height || x >= self.width {
            return false;
        }
        let index = y * self.width + x;
        let mask_index = index / 64;
        let bit_index = index % 64;
        (self.masks[mask_index] & (1u64 << bit_index)) != 0
    }

    fn set_visited(&mut self, y: usize, x: usize) {
        if y >= self.height || x >= self.width {
            return;
        }
        let index = y * self.width + x;
        let mask_index = index / 64;
        let bit_index = index % 64;
        self.masks[mask_index] |= 1u64 << bit_index;
    }
}

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

    // pattern/expression 구분 실수
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
    let mut swan = Vec::with_capacity(2); // 백조 위치
    let mut water = Vec::with_capacity(r * c); // 물 위치
    let mut to_water = Vec::with_capacity(r * c); // 녹을 빙판 위치 정보
    let mut set = UnionFind::new(r * c);
    let mut visited = VisitedMask::new(r, c);
    let mut days = 0;

    // map
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

    // 초반 물 집합 형성
    // 녹을 빙판 조사
    while let Some(value) = water.pop() {
        let y = value / c;
        let x = value % c;
        if visited.is_visited(y, x) {
            continue;
        }
        visited.set_visited(y, x);
        for i in 0..4 {
            let ny = y as isize + DY[i];
            let nx = x as isize + DX[i];
            if nx < 0 || c as isize <= nx || ny < 0 || r as isize <= ny {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if visited.is_visited(ny, nx) {
                continue;
            }
            // 빙판
            if map[ny][nx] == 'X' {
                to_water.push(c * ny as usize + nx as usize);
            }
            // 물
            else {
                set.union(value, c * ny as usize + nx as usize);
            }
        }
    }

    // process
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
                    if visited.is_visited(y, x) {
                        continue;
                    }
                    visited.set_visited(y, x);
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

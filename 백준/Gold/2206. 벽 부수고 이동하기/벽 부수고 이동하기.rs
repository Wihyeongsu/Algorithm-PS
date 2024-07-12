use std::{
    collections::VecDeque,
    io::{self, stdin, BufRead, Read},
    vec,
};

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

    fn set_visited(&mut self, y: usize, x: usize, state: bool) {
        if y >= self.height || x >= self.width {
            return;
        }
        let index = y * self.width + x;
        let mask_index = index / 64;
        let bit_index = index % 64;
        self.masks[mask_index] = match state {
            true => self.masks[mask_index] | 1_u64 << bit_index,
            false => self.masks[mask_index] & !(1_u64 << bit_index),
        }
    }
    fn print_2d(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_visited(y, x) {
                    print!("◼ "); // 방문한 위치
                } else {
                    print!("◻ "); // 방문하지 않은 위치
                }
            }
            println!(); // 줄 바꿈
        }
    }
}

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut lines = stdin.lines();

    let mut buffer = String::new();
    let buffer = lines.next().unwrap()?;
    let mut buffer = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let (n, m) = (buffer.next().unwrap(), buffer.next().unwrap());
    let mut map = VisitedMask::new(n, m);
    for i in 0..n {
        if let Some(line) = lines.next() {
            let mut line = line?.parse::<String>().unwrap();
            let mut line = line.chars();
            for j in 0..m {
                let state = line.next().unwrap() == '1';
                // println!("{i} {j} {state}");
                map.set_visited(i, j, state);
            }
        } else {
            break;
        }
    }
    // map.print_2d();
    // println!("{:?}", walls);
    // // bfs
    if (n, m) == (1, 1) {
        println!("1");
        return Ok(());
    }
    let mut answer = n * m + 1;
    let mut visited = vec![vec![0; m]; n];
    let mut visited_wall = vec![vec![0; m]; n];
    let mut q = VecDeque::new(); // Vec -> VecDeque
    q.push_back((0, false));
    visited[0][0] = 1;

    'bfs: while let Some((index, wall)) = q.pop_front() {
        let (y, x) = (index / m, index % m);

        for i in 0..4 {
            let (ny, nx) = (y as isize + DY[i], x as isize + DX[i]);
            if !(0..n as isize).contains(&ny) || !(0..m as isize).contains(&nx) {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);

            match wall {
                true => {
                    if (ny, nx) == (n - 1, m - 1) {
                        answer = answer.min(visited_wall[y][x] + 1);
                        break;
                    }
                    if map.is_visited(ny, nx) {
                        continue;
                    } else {
                        if (visited_wall[ny][nx] > 0
                            && visited_wall[ny][nx] <= visited_wall[y][x] + 1)
                            || (visited[ny][nx] > 0 && visited[ny][nx] <= visited_wall[y][x] + 1)
                        {
                            continue;
                        }
                        visited_wall[ny][nx] = visited_wall[y][x] + 1;
                        q.push_back((ny * m + nx, wall));
                    }
                }
                false => {
                    if (ny, nx) == (n - 1, m - 1) {
                        answer = answer.min(visited[y][x] + 1);
                        break;
                    }
                    if map.is_visited(ny, nx) {
                        if visited_wall[ny][nx] > 0 && visited_wall[ny][nx] <= visited[y][x] + 1 {
                            continue;
                        }
                        visited_wall[ny][nx] = visited[y][x] + 1;
                        q.push_back((ny * m + nx, true));
                    } else {
                        if visited[ny][nx] > 0 && visited[ny][nx] <= visited[y][x] + 1 {
                            continue;
                        }
                        visited[ny][nx] = visited[y][x] + 1;
                        q.push_back((ny * m + nx, wall));
                    }
                }
            }
        }
    }

    // println!("{:?}", visited);
    // println!("{:?}", visited_wall);
    if answer == n * m + 1 {
        println!("-1");
    } else {
        println!("{answer}");
    }
    Ok(())
}

use std::io::{self, BufRead};

// padding으로 맵 외부 전처리
// bfs를 통해 맵을 조사
// 문을 만나면 키를 소유할 때까지 탐색 보류
// 키 획득 시 해당 문의 위치를 큐에 삽입

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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let mut iter = buffer.split_whitespace();
        let h: usize = iter.next().unwrap().parse::<usize>().unwrap() + 2;
        let w: usize = iter.next().unwrap().parse::<usize>().unwrap() + 2;

        let mut answer = 0;
        let mut map = vec![0u8; h * w];
        let mut key = [false; 26];
        let mut door: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut outer: Vec<usize> = Vec::new();
        let mut visited = VisitedMask::new(h, w);

        // 맵 생성
        for i in 1..h - 1 {
            buffer.clear();
            stdin.read_line(&mut buffer)?;
            for (j, &c) in buffer.trim().as_bytes().iter().enumerate() {
                map[i * w + j + 1] = c;
            }
        }

        // 키 등록
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        for &c in buffer.trim().as_bytes() {
            if c == b'0' {
                break;
            }
            key[(c - b'a') as usize] = true;
        }

        // BFS
        outer.push(0);
        while let Some(pos) = outer.pop() {
            let (y, x) = (pos / w, pos % w);
            // if visited.is_visited(y, x) {
            //     continue;
            // }
            visited.set_visited(y, x);

            for i in 0..4 {
                let ny = y as isize + DY[i];
                let nx = x as isize + DX[i];
                if ny < 0 || ny >= h as isize || nx < 0 || nx >= w as isize {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                let new_pos = ny * w + nx;
                if visited.is_visited(ny, nx) {
                    continue;
                }

                match map[new_pos] {
                    b'*' => continue,
                    b'$' => {
                        visited.set_visited(ny, nx);
                        answer += 1;
                        outer.insert(0, new_pos);
                    }
                    b'a'..=b'z' => {
                        visited.set_visited(ny, nx);
                        let key_num = (map[new_pos] - b'a') as usize;
                        key[key_num] = true;
                        outer.extend_from_slice(&door[key_num]);
                        door[key_num].clear();
                        outer.push(new_pos);
                    }
                    b'A'..=b'Z' => {
                        let key_num = (map[new_pos] - b'A') as usize;
                        if key[key_num] {
                            outer.push(new_pos);
                        } else {
                            door[key_num].push(new_pos);
                        }
                    }
                    _ => outer.push(new_pos),
                }
            }
        }

        println!("{}", answer);
    }

    Ok(())
}

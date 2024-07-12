use std::{
    collections::VecDeque,
    io::{self, stdin, BufRead, Read},
    vec,
};

// visited에 지나온 경로를 저장하여 방문정보, 비용, 경로를 관리 -> 메모리 초과
// 동생 위치에서 시작하는 역과정
// visited에 비용과 이전 위치를 저장하여 관리
// 동생이 수빈보다 작은위치 고려

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut lines = stdin.lines();

    let mut buffer = String::new();
    let buffer = lines.next().unwrap()?;
    let mut buffer = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let (n, k) = (buffer.next().unwrap(), buffer.next().unwrap());

    // bfs

    let mut q = VecDeque::new();
    let mut visited = vec![(0, 0); 2 * k.max(n) + 1]; // (비용, 이전위치)

    q.push_back(k);
    visited[k] = (1, -1); // 시작위치의 이전위치 -1
    'bfs: while let Some(x) = q.pop_front() {
        if x == n {
            println!("{}", visited[x].0 - 1);
            let mut index = x as isize;
            while index != -1 {
                print!("{index} ");
                index = visited[index as usize].1;
            }
            break 'bfs;
        }
        for i in 0..3 {
            let nx = match i {
                0 => x as isize - 1,
                1 => x as isize + 1,
                2 if x % 2 == 0 => x as isize / 2,
                _ => -1, // error
            };
            if nx < 0 {
                continue;
            }
            let nx = nx as usize;
            // 도착지 초과한 상황에서 위치 감소
            if x < n && nx < x && n < k {
                continue;
            }

            if visited[nx].0 > 0 && visited[nx].0 <= visited[x].0 + 1 {
                continue;
            }

            visited[nx] = (visited[x].0 + 1, x as isize);
            q.push_back(nx);
        }
    }
    // println!("{:?}", visited);
    Ok(())
}

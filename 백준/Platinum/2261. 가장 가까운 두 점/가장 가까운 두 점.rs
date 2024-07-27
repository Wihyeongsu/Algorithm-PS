use std::{
    io::{self, Read},
    str::SplitAsciiWhitespace,
    usize::MAX,
};

// x좌표 기준 정렬
// 이진 트리 형식으로 분할
// 좌표 집합의 크기가 3 이하가 되었을 때 해당 집합의 최소 길이 계산
// 왼쪽 노드, 오른쪽 노드의 길이를 비교하여 최소 길이 선발
// 분할 기준의 양쪽으로 각각 최소 길이만큼의 범위에 포함되는 좌표들의 집합 구성
// 새로운 집합에 대해서 y좌표 기준 정렬
// 좌표를 y좌표에 대해 오름차순으로 탐색하여 두 좌표의 y축 방향 길이가 최소 길이 이내이면 최소길이 계산하여 두 집합의 합에 대한 최소 길이 계산

// 7% 시간초과 -> 최소길이의 루트로 범위 비교했던 것을 제곱 결과 그대로 사용하여 비교하니 통과

fn sq_dist((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as usize
}

fn divide_conquer(cord_x: &mut [(isize, isize)]) -> usize {
    if cord_x.len() <= 3 {
        let mut min_dist = MAX;
        for i in 0..cord_x.len() {
            for j in 0..i {
                min_dist = min_dist.min(sq_dist(cord_x[i], cord_x[j]));
            }
        }
        return min_dist;
    }

    let mid = cord_x.len() / 2;
    let mid_cord = cord_x[mid];
    let (mut left_cord_x, mut right_cord_x) = cord_x.split_at_mut(mid);

    let lv = divide_conquer(&mut left_cord_x);
    let rv = divide_conquer(&mut right_cord_x);
    let mut min_dist = lv.min(rv);

    // println!("{cord_in_band:?}");
    let mut cord_y: Vec<(isize, isize)> = cord_x
        .iter()
        .filter(|&&(x, _)| ((mid_cord.0 - x).pow(2) as usize) < min_dist)
        .cloned()
        .collect();
    cord_y.sort_by_key(|&(_, y)| y);

    for i in 0..cord_y.len() {
        let cord1 = cord_y[i];
        for j in i + 1..cord_y.len() {
            let cord2 = cord_y[j];

            if (cord2.1 - cord1.1).pow(2) as usize > min_dist {
                break;
            }
            min_dist = min_dist.min(sq_dist(cord1, cord2));
        }
    }
    min_dist
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace();
    let input_unwrap = |x: &mut SplitAsciiWhitespace| x.next().unwrap().parse::<usize>().unwrap();
    let n = input_unwrap(&mut input);
    drop(input_unwrap);

    let input_unwrap = |x: &mut SplitAsciiWhitespace| x.next().unwrap().parse::<isize>().unwrap();
    let mut cord = Vec::with_capacity(n); // (x, y)
    for _ in 0..n {
        cord.push((input_unwrap(&mut input), input_unwrap(&mut input)))
    }

    cord.sort_by_key(|&(x, _)| x);
    let mut cord_x: Vec<(isize, isize)> = cord.clone();

    let min_dist = divide_conquer(&mut cord_x);

    println!("{min_dist}");
    // println!("{cord:?}");

    Ok(())
}

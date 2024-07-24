use std::{
    io::{self, Read},
    str::SplitAsciiWhitespace,
};

// 2x2 ㄱ자 블럭 -> 4x4 ㄱ자 블럭 -> ...
// 2^n x 2^n ㄱ자 블럭 4개 -> 2^n+1 x 2^n+1 ㄱ자 블럭 1개
// 2x2 -> ㄱ자 블럭 + 배수구

// ㄱ자 블럭과 정사각형으로 분리
// 배수구를 포함한 정사각형을 계속 ㄱ자 블럭과 정사각형으로 분리하여 배수구 단일 블럭이 남을 때까지 진행

// ㄱ자 블럭 분할
fn giyeok_block(
    s_x: usize,
    e_x: usize,
    s_y: usize,
    e_y: usize,
    num: &mut usize,
    size: usize,
    dir: usize,
    floor: &mut Vec<Vec<isize>>,
) {
    if size == 2 {
        let mut cnt = 0;
        for i in s_y..=e_y {
            for j in s_x..=e_x {
                cnt += 1;
                if cnt == dir {
                    continue;
                }
                floor[i][j] = *num as isize;
            }
        }
        *num += 1;
        return;
    }
    let mid_x = (s_x + e_x) / 2;
    let mid_y = (s_y + e_y) / 2;

    // center block
    giyeok_block(
        mid_x + 1 - size / 4,
        mid_x + size / 4,
        mid_y + 1 - size / 4,
        mid_y + size / 4,
        num,
        size / 2,
        dir,
        floor,
    );
    if dir != 1 {
        giyeok_block(s_x, mid_x, s_y, mid_y, num, size / 2, 4, floor);
    }
    if dir != 2 {
        giyeok_block(mid_x + 1, e_x, s_y, mid_y, num, size / 2, 3, floor);
    }
    if dir != 3 {
        giyeok_block(s_x, mid_x, mid_y + 1, e_y, num, size / 2, 2, floor);
    }
    if dir != 4 {
        giyeok_block(mid_x + 1, e_x, mid_y + 1, e_y, num, size / 2, 1, floor);
    }
}

// 배수구 블럭 분리
fn divide(
    s_x: usize,
    e_x: usize,
    s_y: usize,
    e_y: usize,
    hole_x: usize,
    hole_y: usize,
    num: &mut usize,
    size: usize,
    floor: &mut Vec<Vec<isize>>,
) {
    if size == 1 {
        floor[hole_y][hole_x] = -1;
        return;
    }

    let mid_x = (s_x + e_x) / 2;
    let mid_y = (s_y + e_y) / 2;

    if (s_x..=mid_x).contains(&hole_x) && (s_y..=mid_y).contains(&hole_y) {
        // 1
        giyeok_block(s_x, e_x, s_y, e_y, num, size, 1, floor);
        divide(s_x, mid_x, s_y, mid_y, hole_x, hole_y, num, size / 2, floor);
    } else if (mid_x + 1..=e_x).contains(&hole_x) && (s_y..=mid_y).contains(&hole_y) {
        // 2
        giyeok_block(s_x, e_x, s_y, e_y, num, size, 2, floor);
        divide(
            mid_x + 1,
            e_x,
            s_y,
            mid_y,
            hole_x,
            hole_y,
            num,
            size / 2,
            floor,
        );
    } else if (s_x..=mid_x).contains(&hole_x) && (mid_y + 1..=e_y).contains(&hole_y) {
        // 3
        giyeok_block(s_x, e_x, s_y, e_y, num, size, 3, floor);
        divide(
            s_x,
            mid_x,
            mid_y + 1,
            e_y,
            hole_x,
            hole_y,
            num,
            size / 2,
            floor,
        );
    } else if (mid_x + 1..=e_x).contains(&hole_x) && (mid_y + 1..=e_y).contains(&hole_y) {
        // 4
        giyeok_block(s_x, e_x, s_y, e_y, num, size, 4, floor);
        divide(
            mid_x + 1,
            e_x,
            mid_y + 1,
            e_y,
            hole_x,
            hole_y,
            num,
            size / 2,
            floor,
        );
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace();
    let input_unwrap = |x: &mut SplitAsciiWhitespace| x.next().unwrap().parse::<usize>().unwrap();
    let k = input_unwrap(&mut input);
    let x = input_unwrap(&mut input) - 1;
    let y = input_unwrap(&mut input) - 1;
    let size = 2_usize.pow(k as u32);

    let mut floor = vec![vec![0; size]; size];

    divide(0, size - 1, 0, size - 1, x, y, &mut 1, size, &mut floor);

    //print
    for i in (0..size).rev() {
        for j in 0..size {
            print!("{} ", floor[i][j]);
        }
        println!("");
    }

    Ok(())
}

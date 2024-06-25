use std::{
    arch::x86_64::_SIDD_LEAST_SIGNIFICANT,
    io::{stdin, Read},
    ops::RemAssign,
    vec,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<isize>);
    let t = input.next().unwrap();
    let a = input.next().unwrap() as usize;
    let set_a: Vec<isize> = input.by_ref().take(a).collect();
    let b = input.next().unwrap() as usize;
    let set_b: Vec<isize> = input.collect();
    let mut sum_a = Vec::with_capacity(a * (a + 1) / 2);
    let mut sum_b = Vec::with_capacity(b * (b + 1) / 2);
    for i in 0..a {
        let mut sum = set_a[i];
        sum_a.push(sum);
        for j in i + 1..a {
            sum += set_a[j];
            sum_a.push(sum);
        }
    }
    for i in 0..b {
        let mut sum = set_b[i];
        sum_b.push(set_b[i]);
        for j in i + 1..b {
            sum += set_b[j];
            sum_b.push(sum);
        }
    }
    sum_a.sort();
    sum_b.sort();
    let mut answer: usize = 0;
    let mut left: isize = 0;
    let mut right: isize = sum_b.len() as isize - 1;
    while left < sum_a.len() as isize && 0 <= right {
        let l_sum = sum_a[left as usize];
        let r_sum = sum_b[right as usize];
        let sum = l_sum + r_sum;
        if sum == t {
            let mut l_cnt = 0;
            let mut r_cnt = 0;
            while left < sum_a.len() as isize && l_sum == sum_a[left as usize] {
                left += 1;
                l_cnt += 1;
            }
            while 0 <= right && r_sum == sum_b[right as usize] {
                right -= 1;
                r_cnt += 1;
            }
            answer += l_cnt * r_cnt;
        } else if sum < t {
            left += 1;
        } else if sum > t {
            right -= 1;
        }
    }
    println!("{answer}");
    // println!("{:#?}", sum_a);
    // println!("{:#?}", sum_b);
}

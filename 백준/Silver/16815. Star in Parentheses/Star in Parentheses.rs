use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("*");
    let l = input.next().unwrap();
    let r = input.next().unwrap();
    let mut l_cnt = 0;
    let mut r_cnt = 0;
    for i in l.chars() {
        if i == '(' {
            l_cnt += 1;
        }
        else if i == ')' && l_cnt > 0 {
            l_cnt -= 1;
        }
    }
    let mut open = 0;
    for i in r.chars() {
        if i == '(' && r_cnt > 0{
            r_cnt -= 1;
        }
        else if i == ')' {
            r_cnt += 1;
        }
    }
    print!("{}", std::cmp::min(l_cnt, r_cnt));
}
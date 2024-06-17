use std::io::{stdin, Read};

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<String>);

    // minimum unit 1001, 01
    'outer: for _ in 0..t {
        let spread = input.next().unwrap();
        let mut s ='#';
        let mut zero_cnt = 0;
        let mut cnt = 0;
        'inner: for i in spread.chars() {
            cnt += 1;
            if s == '#' || (s == '3' && i == '0') {
                s = i;
            }
            else if (s == '0' && i == '1') || (s == '5' && i == '1' /* 01 pattern*/) {
                s = '#';
            }
            else if (s == '1' && i == '0') || (s == '5' && i == '0' /*100+1+ pattern*/) {
                zero_cnt += 1;
                if zero_cnt >= 2 { // 100 ensure
                    s = '2';
                    zero_cnt = 0;
                }
            }
            else if (s == '0' && i == '0') || (s == '1' && i == '1') {
                println!("NO");
                continue 'outer;
            }
            else if s == '2' && i == '1' { // 100+1 ensure
                s = '3';
            }
            else if s == '3' && i == '1' { // 100+1+ ensure
                s = '4';
            }
            else if s == '4' && i == '0' { // next 100+1+|01 pattern
                s = '5';
                zero_cnt += 1;
            }
        }
        if s == '0' || s == '1' || s == '2' {println!("NO");}
        else {println!("YES");}
    }
}
use std::io::stdin;

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<usize>().unwrap();
    let mut remainder = 0;
    loop {
        if input >= 3 {
            remainder = input % 3;
            input /= 3;
        }
        if input == 2 || remainder == 2 || input == 0{
            print!("NO");
            break;
        }
        else if input == 1  {
            print!("YES");
            break;
        }
    }
}
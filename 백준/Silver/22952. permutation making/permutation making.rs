use std::io::stdin;

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();
    print!("{input} ");
    for i in (1..=input/2) {
        if i == input - i {
            print!("{i}");
        }
        else {
            print!("{i} {} ", input - i);
        }
    }
}
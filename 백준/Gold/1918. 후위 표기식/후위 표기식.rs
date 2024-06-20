use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().chars();
    let mut operator: Vec<char> = Vec::new();
    let priority = HashMap::from([('*', 1), ('/', 1), ('+', 2), ('-', 2), ('(', 3)]);

    loop {
        let sub_token = input.next();
        let token = match sub_token {
            None => '#',
            _ => sub_token.unwrap(),
        };
        let stack_token = operator.pop();
        if token == '#' && stack_token == None {
            break;
        };
        match token {
            '#' if stack_token != None => {
                let stack_token = stack_token.unwrap();
                print!("{stack_token}");
            }
            '*' | '/' | '+' | '-' if stack_token == None => operator.push(token),
            '(' => {
                if stack_token != None {
                    operator.push(stack_token.unwrap());
                }
                operator.push(token);
            }
            '*' | '/' | '+' | '-' => {
                let mut stack_token = stack_token.unwrap();

                loop {
                    match priority.get(&stack_token).unwrap() <= priority.get(&token).unwrap() {
                        true => {
                            print!("{stack_token}");
                            let sub_stack_token = operator.pop();
                            stack_token = match sub_stack_token {
                                None => {
                                    break;
                                }
                                _ => sub_stack_token.unwrap(),
                            }
                        }
                        false => {
                            operator.push(stack_token);
                            break;
                        }
                    }
                }
                operator.push(token);
            }
            ')' => {
                let mut stack_token = stack_token.unwrap();
                while stack_token != '(' {
                    print!("{stack_token}");
                    stack_token = operator.pop().unwrap();
                }
            }
            _ => {
                print!("{token}");
                if stack_token != None {
                    operator.push(stack_token.unwrap());
                }
            }
        }
    }
}

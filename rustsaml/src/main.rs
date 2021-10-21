use std::io;

mod shared;

fn main() {
    let mut inputs: Vec<String> = Vec::new();

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let temp = input.trim();
                    if temp.is_empty() {
                        break;
                    }
                    inputs.push(String::from(temp));
                    input = String::new();
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    for input in inputs {
        println!("{}", input);
    }
}

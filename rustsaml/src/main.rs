use std::io;

mod shared;
mod parser;

fn main() {
    let mut inputs: Vec<shared::KeyValuePair> = Vec::new();

    let mut input = String::new();
    let mut done = false;

    while !done {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let temp = input.trim();
                    if temp.is_empty() {
                        done = true;
                        continue;
                    }
                    let result = parser::awsaml::parse_key_value(temp.to_string());
                    if result.is_ok() {
                        inputs.push(result.unwrap())
                    }
                    input = String::new();
                }
            }
            Err(_) => {
                done = true;
            }
        }
    }

    for input in inputs.iter() {
        let (key, value) = input.deconstruct();
        println!("{}: {}", key, value);
    }
}

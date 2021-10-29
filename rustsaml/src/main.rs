use std::io;

mod parser;
mod shared;
mod shell_exporter;

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

    match shell_exporter::exporter::get_exporter("powershell") {
        Ok(export_fn) => {
            for pair in inputs {
                match export_fn(pair) {
                    Ok(output) => println!("{}", output),
                    Err(_) => {}
                };
            }
        }
        Err(_) => {}
    }
}

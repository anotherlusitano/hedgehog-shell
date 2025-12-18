#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    let valid_commands = ["exit"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let command = valid_commands
            .iter()
            .find(|command| command.to_string() == input.trim());

        match command {
            Some(c) => {
                if c == &"exit" {
                    break;
                }
            }
            None => println!("{}: command not found", input.trim()),
        };

        input.clear();
    }
}

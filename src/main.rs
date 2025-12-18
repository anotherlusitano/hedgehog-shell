#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Debug)]
struct Command<'a> {
    name: &'a str,
    args: Option<&'a str>,
}

fn main() {
    let mut input = String::new();

    let valid_commands = ["exit", "echo"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let tokens = input.trim().split_once(' ');

        // TODO: some commands don't allow args so we should verify that somehow
        let cmd = if let Some((cname, args)) = tokens {
            Command {
                name: cname,
                args: Some(args),
            }
        } else {
            Command {
                name: input.trim(),
                args: None,
            }
        };

        if !valid_commands.contains(&cmd.name) {
            println!("{}: command not found", input.trim())
        }

        match cmd.name {
            "exit" => break,
            "echo" => println!("{}", cmd.args.unwrap()),
            _ => {
                input.clear(); // Clear the errors
                continue;
            }
        }

        input.clear(); // Don't forget to clear the buffer
    }
}

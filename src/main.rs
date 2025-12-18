#[allow(unused_imports)]
use std::io::{self, Write};

const VALID_COMMANDS: [ValidCommand; 3] = [
    ValidCommand("exit", false),
    ValidCommand("echo", true),
    ValidCommand("type", true),
];

/// 0 = Name ; 1 = have args?
#[derive(Debug, PartialEq)]
struct ValidCommand<'a>(&'a str, bool);

fn is_valid_command(command_name: &'_ str) -> Result<ValidCommand<'_>, &'_ str> {
    for command in VALID_COMMANDS {
        if command.0 == command_name {
            return Ok(command);
        }
    }

    Err("command not found")
}

fn main() {
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<&str> = input.trim().split(' ').collect();

        let (cmd, args) = tokens.split_first().unwrap();

        match is_valid_command(cmd) {
            Ok(cmd) => {
                // If the Command doesn't need args but there are args
                if !cmd.1 && tokens.len() > 1 {
                    println!("{}: too many arguments", input.trim());
                    input.clear(); // Clear the errors
                    continue;
                }
                // If the Command need args but there are no args
                if cmd.1 && tokens.len() == 1 {
                    println!("{}: too few arguments", input.trim());
                    input.clear(); // Clear the errors
                    continue;
                }

                match cmd.0 {
                    "exit" => break,
                    "echo" => println!("{}", args.join(" ")),
                    "type" => {
                        let have_cmd = VALID_COMMANDS
                            .iter()
                            .find(|cmd| &cmd.0 == args.first().unwrap());

                        match have_cmd {
                            Some(cmd) => println!("{} is a shell builtin", cmd.0),
                            None => println!("{}: not found", args.join(" ")),
                        }
                    }
                    _ => {
                        input.clear(); // Clear the errors
                        continue;
                    }
                }
            }
            Err(e) => println!("{}: {}", input.trim(), e),
        }

        input.clear(); // Don't forget to clear the buffer
    }
}

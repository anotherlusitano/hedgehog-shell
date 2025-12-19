#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, ffi::OsString, os::unix::fs::PermissionsExt};

const BUILTIN_COMMANDS: [BuiltinCommand; 3] = [
    BuiltinCommand("exit", false),
    BuiltinCommand("echo", true),
    BuiltinCommand("type", true),
];

/// 0 = Name ; 1 = have args?
#[derive(Debug, PartialEq)]
struct BuiltinCommand<'a>(&'a str, bool);

fn find_builtin_command(command_name: &'_ str) -> Option<BuiltinCommand<'_>> {
    BUILTIN_COMMANDS
        .into_iter()
        .find(|command| command.0 == command_name)
}

fn main() {
    // Executables of the $PATH
    let executables = get_all_executables();

    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<&str> = input.trim().split(' ').collect();

        let (cmd, args) = tokens.split_first().unwrap();

        match find_builtin_command(cmd) {
            Some(cmd) => {
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
                        for command in args {
                            let builtin_cmd = BUILTIN_COMMANDS.iter().find(|cmd| &cmd.0 == command);

                            match builtin_cmd {
                                Some(cmd) => println!("{} is a shell builtin", cmd.0),
                                None => {
                                    search_executable(command, &executables);
                                }
                            }
                        }
                    }
                    _ => {
                        input.clear(); // Clear the errors
                        continue;
                    }
                }
            }
            None => {
                search_executable(input.trim(), &executables);
            }
        }

        input.clear(); // Don't forget to clear the buffer
    }
}

/// Will search for the executable inside a list of executables
/// If didn't find any, will print "command not found"
fn search_executable(exe: &str, executables: &Vec<(OsString, String)>) {
    for executable in executables {
        if *executable.0 == *exe {
            println!("{} is {}", executable.0.display(), executable.1);
            break;
        } else if executables.last() == Some(executable) {
            println!("{}: not found", exe);
        }
    }
}

/// Splits the $PATH into individual `Path` entries
/// For each entry, verifies whether the files inside are executable
/// Returns a `Vec` containing the name and the path of the executables
fn get_all_executables() -> Vec<(OsString, String)> {
    let mut all_executables = Vec::new();
    let key = "PATH";

    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let entries = path.read_dir();

                if entries.is_err() {
                    continue;
                }

                for entry in entries.unwrap() {
                    let entry_path = entry.unwrap().path();

                    if entry_path.is_dir() || entry_path.is_symlink() {
                        continue;
                    }

                    let file_permission = &entry_path.metadata().unwrap().permissions().mode();

                    // if File is executable
                    if file_permission & 0o111 != 0 {
                        let executable_path = entry_path.to_str().unwrap();
                        let executable_name = entry_path.file_name().unwrap();

                        all_executables
                            .push((executable_name.to_os_string(), executable_path.to_owned()));
                    }
                }
            }
        }
        None => println!("Why the hell you don't have {key} in your environment???"),
    }

    all_executables
}

use std::{
    collections::HashMap,
    io::{self, Write},
};

mod commands;

fn main() {
    let mut alias_map: HashMap<String, String> = HashMap::new();
    println!("Type 'help' for list of all commands available");

    // Main REPL loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let mut args = parts;

            // Check for aliases
            let resolved_command = match alias_map.get(command) {
                Some(alias) => alias.as_str(),
                None => command,
            };

            match resolved_command {
                "alias" => {
                    // Handle alias command
                    if let (Some(alias), Some(cmd)) = (args.next(), args.next()) {
                        alias_map.insert(alias.to_string(), cmd.to_string());
                    } else {
                        eprintln!("Usage: alias <alias_name> <command>");
                    }
                }
                "help" => {
                    commands::display_help();
                    previous_command = None;
                }
                "cd" => {
                    commands::change_directory(&mut args.collect::<Vec<&str>>());
                    previous_command = None;
                }
                "dir" => {
                    commands::list_directory();
                    previous_command = None;
                }
                "echo" => {
                    commands::echo(args);
                    previous_command = None;
                }
                "rm" => {
                    commands::remove_file(args);
                    previous_command = None;
                }
                "pwd" => {
                    commands::print_working_directory();
                    previous_command = None;
                }
                "mkdir" => {
                    commands::make_directory(args);
                    previous_command = None;
                }
                "rmdir" => {
                    commands::remove_directory(args);
                    previous_command = None;
                }
                "exit" => return,
                _ => {
                    println!("Command not found")
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}

use std::{
    collections::{HashMap, VecDeque},
    io::{self, Write},
    thread,
};

mod commands;

fn main() {
    let mut alias_map: HashMap<String, String> = HashMap::new();
    let mut background_jobs: VecDeque<thread::JoinHandle<commands::BackgroundJob>> =
        VecDeque::new();
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
                    commands::make_alias(&mut alias_map, &mut args);
                    previous_command = None;
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
                "bg" => commands::exec_background_job(&mut args, &mut background_jobs),
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

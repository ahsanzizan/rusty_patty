use std::io::{self, Write};

mod commands;

fn main() {
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
            let args = parts;

            match command {
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

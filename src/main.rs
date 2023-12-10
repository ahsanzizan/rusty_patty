use std::io::{self, Write};
use std::process::Stdio;

mod commands;

// Main REPL loop
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
                    commands::rm(args);
                    previous_command = None;
                }
                "pwd" => {
                    commands::pwd();
                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(), |output: std::process::Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    if let Some(output) =
                        commands::execute_external_command(command, args, stdin, stdout)
                    {
                        previous_command = Some(output);
                    } else {
                        previous_command = None;
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}

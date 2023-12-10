use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    // Main REPL loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Split input into commands separated by pipe ('|') and make it peekable
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        // Process each command
        while let Some(command) = commands.next() {
            // Split command into command and arguments
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            // Match the command type
            match command {
                "cd" => {
                    // Change directory command
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    // Reset previous command since 'cd' doesn't produce output
                    previous_command = None;
                }
                "exit" => return, // Exit the REPL
                command => {
                    // External command
                    let stdin: Stdio = previous_command
                        .map_or(Stdio::inherit(), |output: std::process::Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    // Spawn the command process
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            // Update previous command with the new process
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            // Reset previous command on error
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        // Wait for the final command to finish before prompting for new input
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}

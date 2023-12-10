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
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Split input into commands separated by pipe ('|') and make it peekable
        let mut commands: std::iter::Peekable<std::str::Split<'_, &str>> =
            input.trim().split(" | ").peekable();
        let mut previous_command: Option<std::process::Child> = None;

        // Process each command
        while let Some(command) = commands.next() {
            // Split command into command and arguments
            let mut parts: std::str::SplitWhitespace<'_> = command.trim().split_whitespace();
            let command: &str = parts.next().unwrap();
            let args: std::str::SplitWhitespace<'_> = parts;

            // Match the command type
            match command {
                "cd" => {
                    // Change directory command
                    let new_dir: &str = args.peekable().peek().map_or("/", |x| *x);
                    let root: &Path = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    // Reset previous command since 'cd' doesn't produce output
                    previous_command = None;
                }
                "dir" => {
                    // Directory listing command
                    let current_dir: std::path::PathBuf = env::current_dir().unwrap();
                    if let Ok(entries) = current_dir.read_dir() {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                println!("{}", entry.file_name().to_string_lossy());
                            }
                        }
                    } else {
                        eprintln!("Error listing directory");
                    }

                    // Reset previous command since 'dir' doesn't produce output for piping
                    previous_command = None;
                }
                "echo" => {
                    // Echo command
                    let output: String = args.collect::<Vec<&str>>().join(" ");
                    println!("{}", output);

                    // Reset previous command since 'echo' doesn't produce output for piping
                    previous_command = None;
                }
                "exit" => return, // Exit the REPL
                command => {
                    // External command
                    let stdin: Stdio = previous_command
                        .map_or(Stdio::inherit(), |output: std::process::Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                    let stdout: Stdio = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    // Spawn the command process
                    let output: Result<std::process::Child, io::Error> = Command::new(command)
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

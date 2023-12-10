use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

// Function to handle the "cd" command
fn change_directory(args: &mut Vec<&str>) {
    // Extract the new directory from the arguments
    let new_dir: &str = args.iter().peekable().peek().map_or("/", |x| **x);
    let root: &Path = Path::new(new_dir);

    // Attempt to change the current directory
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

// Function to handle the "dir" command
fn list_directory() {
    // Get the current directory
    let current_dir: std::path::PathBuf = env::current_dir().unwrap();

    // Attempt to list entries in the directory
    if let Ok(entries) = current_dir.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    } else {
        eprintln!("Error listing directory");
    }
}

// Function to handle the "echo" command
fn echo(args: std::str::SplitWhitespace<'_>) {
    // Concatenate the arguments and print the output
    let output: String = args.collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}

// Function to handle the "rm" command
fn rm(args: std::str::SplitWhitespace<'_>) {
    // Extract the file path from the args
    let file_path: Option<&str> = args.peekable().peek().map(|x| *x);

    // Check if a file path is provided
    match file_path {
        Some(path) => {
            if let Err(e) = std::fs::remove_file(path) {
                eprintln!("{}", e);
            }
        }
        None => {
            eprintln!("Usage: rm <file_path>");
        }
    }
}

// Function to execute an external command
fn execute_external_command(
    command: &str,
    args: std::str::SplitWhitespace<'_>,
    stdin: Stdio,
    stdout: Stdio,
) -> Option<std::process::Child> {
    // Spawn the external command process
    let output: Result<std::process::Child, io::Error> = Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();

    // Handle the result of spawning the process
    match output {
        Ok(output) => Some(output),
        Err(e) => {
            // Print an error message and return None on error
            eprintln!("{}", e);
            None
        }
    }
}

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
                "cd" => {
                    change_directory(&mut args.collect::<Vec<&str>>());
                    previous_command = None;
                }
                "dir" => {
                    list_directory();
                    previous_command = None;
                }
                "echo" => {
                    echo(args);
                    previous_command = None;
                }
                "rm" => {
                    rm(args);
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

                    if let Some(output) = execute_external_command(command, args, stdin, stdout) {
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

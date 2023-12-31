use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::process::Command;
use std::{env, thread};

// Function to handle the "cd" command
pub fn change_directory(args: &mut Vec<&str>) {
    // Extract the new directory from the arguments
    let new_dir: &str = args.iter().peekable().peek().map_or("/", |x| **x);
    let root: &Path = Path::new(new_dir);

    // Attempt to change the current directory
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}

// Function to handle the "dir" command
pub fn list_directory() {
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
pub fn echo(args: std::str::SplitWhitespace<'_>) {
    // Concatenate the arguments and print the output
    let output: String = args.collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}

// Function to handle the "rm" command
pub fn remove_file(args: std::str::SplitWhitespace<'_>) {
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

// Function to handle the 'pwd' command
pub fn print_working_directory() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        eprintln!("Error retrieving current working directory");
    }
}

// Function to handle the 'mkdir' command
pub fn make_directory(args: std::str::SplitWhitespace<'_>) {
    // Extract the directory path from the arguments
    let dir_path: Option<&str> = args.peekable().peek().map(|x| *x);

    // Check if a directory path is provided
    match dir_path {
        Some(path) => {
            if let Err(e) = std::fs::create_dir(path) {
                eprintln!("{}", e);
            }
        }
        None => {
            eprintln!("Usage: mkdir <directory_path>")
        }
    }
}

// Function to handle the 'rmdir' command
pub fn remove_directory(args: std::str::SplitWhitespace<'_>) {
    let dir_path: Option<&str> = args.peekable().peek().map(|x| *x);
    match dir_path {
        Some(path) => {
            if let Err(e) = std::fs::remove_dir(path) {
                eprintln!("{}", e);
            }
        }
        None => {
            eprintln!("Usage: rmdir <directory_path>");
        }
    }
}

// Function to handle the 'alias' command
pub fn make_alias(
    alias_map: &mut HashMap<String, String>,
    args: &mut std::str::SplitWhitespace<'_>,
) {
    if let (Some(alias), Some(cmd)) = (args.next(), args.next()) {
        alias_map.insert(alias.to_string(), cmd.to_string());
    } else {
        eprintln!("Usage: alias <alias_name> <command>");
    }
}

// Background Job interface
pub struct BackgroundJob {
    pub command: String,
    pub status: Option<std::process::ExitStatus>,
}

// Function to handle the 'bg' command
pub fn exec_background_job(
    args: &mut std::str::SplitWhitespace,
    background_jobs: &mut VecDeque<thread::JoinHandle<BackgroundJob>>,
) {
    let cmd = args.collect::<Vec<&str>>().join(" ");
    let child = thread::spawn(move || {
        let status = Command::new("sh").arg("-c").arg(&cmd).status();
        BackgroundJob {
            command: cmd,
            status: status.ok(),
        }
    });

    background_jobs.push_back(child);
}

// Function to display help information
pub fn display_help() {
    println!("Available commands:");
    println!("  cd <directory>                - Change current directory");
    println!("  dir                           - List entries in the current directory");
    println!("  echo <message>                - Print a message to the console");
    println!("  rm <file_path>                - Remove a file");
    println!("  pwd                           - Get current working directory");
    println!("  mkdir <directory_path>        - Create a directory");
    println!("  rmdir <directory_path>        - Remove a directory");
    println!("  alias <alias_name> <command>  - Make alias of a command");
    println!("  bg <command>                  - Execute command in the background");
    println!("  exit                          - Exit the shell");

    println!("Usage:");
    println!("  Multiple commands can be chained using ' | ' for piping.");
}

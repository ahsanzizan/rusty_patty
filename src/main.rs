use std::io::{self, Write};
use std::process::{exit, Command};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: &str = input.trim();

        if input == "exit" {
            exit(0);
        }

        let mut parts: std::str::SplitWhitespace<'_> = input.split_whitespace();
        let command: &str = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let output: std::process::Output = Command::new(command)
            .args(&args)
            .output()
            .expect("Failed to execute command");

        println!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

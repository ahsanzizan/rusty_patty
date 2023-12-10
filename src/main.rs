use std::io::{self, Write};
use std::process::{exit, Command, Stdio};

fn execute_command(command: &str, args: Vec<&str>, input: Option<Vec<u8>>) -> Vec<u8> {
    let mut child: std::process::Child = Command::new(command)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(input_data) = input {
        if let Some(ref mut child_stdin) = child.stdin.take() {
            child_stdin
                .write_all(&input_data)
                .expect("Failed to write to child stdin");
        }
    }

    let output: std::process::Output = child
        .wait_with_output()
        .expect("Failed to wait for command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    output.stdout
}

fn main() {
    let mut previous_output: Option<Vec<u8>> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: &str = input.trim();

        if input == "exit" {
            exit(0);
        }

        if input.contains('|') {
            let commands: Vec<&str> = input.split('|').map(str::trim).collect();

            for command in commands {
                let parts: Vec<&str> = command.split_whitespace().collect();
                previous_output = Some(execute_command(
                    parts[0],
                    parts[1..].to_vec(),
                    previous_output.take(),
                ));
            }
        } else {
            let parts: Vec<&str> = input.split_whitespace().collect();
            previous_output = Some(execute_command(
                parts[0],
                parts[1..].to_vec(),
                previous_output.take(),
            ));
        }
    }
}

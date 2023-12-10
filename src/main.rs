use std::io::{self, Write};
use std::process::{exit, Command, Stdio};

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
            let mut commands: std::iter::Map<std::str::Split<'_, &str>, fn(&str) -> &str> =
                input.split("|").map(str::trim);
            while let Some(command) = commands.next() {
                let parts: Vec<&str> = command.split_whitespace().collect();
                let mut child: std::process::Child = Command::new(parts[0])
                    .args(&parts[1..])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to spawn command in pipeline");

                if let Some(prev_output) = previous_output.take() {
                    if let Some(ref mut child_stdin) = child.stdin.take() {
                        child_stdin
                            .write_all(&prev_output)
                            .expect("Failed to write to child stdin");
                    }
                }

                let output: std::process::Output = child
                    .wait_with_output()
                    .expect("Failed to wait for command in pipeline");

                println!("{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }

                previous_output = Some(output.stdout);
            }
            continue;
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

        // Update previous_output for non-pipeline commands
        previous_output = Some(output.stdout);
    }
}

use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("Shelly >> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut parts = input.trim().split_whitespace();

        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        let args: Vec<&str> = parts.collect();

        if command == "exit" {
            println!("Exiting Shelly...");
            break;
        }

        println!("Executing: {}", command);
        println!("Arguments: {:?}", args);

        if !command.is_empty() {
            let child = Command::new(command)
                .args(&args)
                .spawn();

            match child {
                Ok(mut child_process) => {
                    let status = child_process.wait().expect("Failed to wait on child");

                    if !status.success() {
                        eprintln!("Command exited with an error status.");
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        eprintln!("Shelly: command not found: {}", command);
                    } else {
                        eprintln!("Shelly: an error occurred: {}", e);
                    }
                }
            }
        }
    }
}

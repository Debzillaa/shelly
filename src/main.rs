use std::io::{self, Write};
use std::process::Command;
use std::env;

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

        match command {
            "exit" => {
                println!("Exiting Shelly...");
                break;
            }
            "cd" => {
                let new_dir = args.get(0).map(|&s| s).unwrap_or("/");
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("Shelly: cd: {}", e);
                }
            }
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("Shelly: pwd: {}", e),
                }
            }
            _ => {
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
}

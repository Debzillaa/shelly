use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::fs::File;

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
                let mut command_args = args.clone();
                let mut redirect_file = None;

                if let Some(pos) = command_args.iter().position(|&r| r == ">") {
                    if let Some(filename) = command_args.get(pos + 1) {
                        match File::create(filename) {
                            Ok(file) => redirect_file = Some(file),
                            Err(e) => eprintln!("Shelly: redirection error: {}", e),
                        }
                        command_args.drain(pos..);
                    }
                }

                let mut child_cmd = Command::new(command);
                child_cmd.args(&command_args);

                if let Some(file) = redirect_file {
                    child_cmd.stdout(Stdio::from(file));
                }

                match child_cmd.spawn() {
                    Ok(mut child_process) => {
                        let _ = child_process.wait();
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("Shelly: command not found: {}", command);
                        } else {
                            eprintln!("Shelly: {}", e);
                        }
                    }
                }
            }
        }
    }
}

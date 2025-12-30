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
                if let Some(pipe_pos) = args.iter().position(|&r| r == "|") {
                    let left_cmd_name = command;
                    let left_args = &args[..pipe_pos];

                    let right_cmd_name = args[pipe_pos + 1];
                    let right_args = &args[pipe_pos + 2..];

                    let left_child = Command::new(left_cmd_name)
                        .args(left_args)
                        .stdout(Stdio::piped())
                        .spawn();

                    match left_child {
                        Ok(left_proc) => {
                            let right_child = Command::new(right_cmd_name)
                                .args(right_args)
                                .stdin(Stdio::from(left_proc.stdout.unwrap()))
                                .spawn();

                            match right_child {
                                Ok(mut right_proc) => {
                                    let _ = right_proc.wait();
                                }
                                Err(e) => eprintln!("Shelly: right command error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Shelly: left command error: {}", e),
                    }
                } else {
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
                        Ok(mut child) => { let _ = child.wait(); }
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
    }
}

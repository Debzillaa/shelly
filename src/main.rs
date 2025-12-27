use std::io::{self, Write};

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
    }
}

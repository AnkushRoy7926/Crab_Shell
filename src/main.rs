use std::io::{self, BufRead, BufReader, Stdout, Write, Stdin};
use std::process::Command;

// use std::iter::Peekable;



/// Splits a command line into tokens, handling simple whitespace separation.
fn parse_command(line: &str) -> Vec<String> {
    line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn main() {

    let mut stdout: Stdout = io::stdout();
    let stdin: Stdin = io::stdin();
    let reader: BufReader<io::Stdin> = BufReader::new(stdin);
    let mut lines: io::Lines<BufReader<io::Stdin>> = reader.lines();

    loop{
        print!("Crab Shell");
        stdout.flush().unwrap();

        match lines.next() {
            Some(Ok(line)) => {
                let input = line.trim();
                if input.is_empty() {
                    continue; // Skip empty lines
                } else if input == "exit" {
                    break;
                } else if input == "help" {
                    println!("Available commands: exit, help");
                } else {
                    let parts = parse_command(input);
                    let command = &parts[0];
                    let args = &parts[1..];

                    let mut child = Command::new(command)
                        .args(args)
                        .spawn();

                    match child {
                        Ok(mut child_proc) => {
                            // Wait for the command to finish and capture the status
                            match child_proc.wait() {
                                Ok(status) => println!("Process exited with: {}", status),
                                Err(e) => eprintln!("Failed to wait on child process: {}", e),
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute {}: {}", command, e);
                        }
                    }

                    println!("Command: {}", command);
                    println!("Args: {:?}", args);
                }
            }
            Some(Err(e)) => {
                eprintln!("Error reading line: {}", e);
            }
            None => {
                break; // EOF reached   
            }
        }
    }
}


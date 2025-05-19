use std::io::{self, BufRead, BufReader, Stdout, Write, Stdin};
use std::process::Command;

/// Splits a command line into tokens, handling simple whitespace separation.
fn parse_command(line: &str) -> Vec<String> {
    line
        .split_whitespace() // Split by whitespace
        .map(|s| s.to_string()) // Convert to String
        .collect() // Collect into a Vec<String>
}

fn main() {

    // Set up the standard output and input
    // This is necessary to ensure that the output is flushed immediately
    // and to read input from the user.
    let mut stdout: Stdout = io::stdout();
    let stdin: Stdin = io::stdin();

    // Create a buffered reader for standard input
    // This allows us to read lines of input efficiently.
    // The BufReader wraps the Stdin handle, allowing us to read lines from it.
    // The lines() method returns an iterator over the lines of input.
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    
    loop{

        print!("lordraleigh>> "); // Prompt the user for input
        stdout.flush().unwrap(); // Flush the output to ensure the prompt is displayed

        // Read a line from the standard input
        // The lines() method returns an iterator over the lines of input.
        // The next() method retrieves the next line, which is an Option<Result<String, io::Error>>.
        match lines.next() {

            // Handle the result of reading a line
            // If the line is Ok, we proceed to process it.
            // If the line is Err, we print an error message.
            Some(Ok(line)) => {

                // Trim the line to remove leading and trailing whitespace
                let input = line.trim();

                // Check if the input is empty
                // If it is empty, we skip to the next iteration of the loop.
                if input.is_empty() {
                    continue; // Skip empty lines
                }
                
                // Check for special commands
                // If the input is "exit", we break out of the loop to terminate the program.
                else if input == "exit" {
                    break;
                }
                
                // Check for help command
                // If the input is "help", we print the available commands.
                // This is a simple command-line help feature.
                else if input == "help" {
                    println!("Available commands: exit, help");
                }
                
                
                // If the input is not empty and not a special command, we proceed to execute it.
                // We split the input into command and arguments using the parse_command function.
                // The parse_command function handles simple whitespace separation.
                // We then create a new Command using the command and arguments.
                // The Command struct is used to spawn child processes.
                else {

                    // Split the input into command and arguments
                    // The parse_command function handles simple whitespace separation.
                    let parts = parse_command(input);

                    let command = &parts[0];
                    let args = &parts[1..];

                    // Execute the command
                    // The Command struct is used to spawn child processes.
                    let mut child = Command::new(command)
                        .args(args)
                        .spawn();

                    // Check if the command was executed successfully
                    // The spawn() method returns a Result<Child, io::Error>.
                    match child {

                        // If the command was executed successfully, we get a Child process.
                        Ok(mut child_proc) => {

                            // Wait for the command to finish and capture the status
                            match child_proc.wait() {

                                // If the command finished successfully, we get an ExitStatus.
                                Ok(status) => continue, //println!("Process exited with: {}", status),

                                // If there was an error waiting for the command, we print an error message.
                                Err(e) => eprintln!("Failed to wait on child process: {}", e),
                            }
                        }

                        // If there was an error executing the command, we print an error message.
                        Err(e) => {
                            eprintln!("Failed to execute {}: {}", command, e);
                        }
                    }

                    // Print the command and arguments for debugging purposes
                    // This is useful for understanding what command was executed.
                    // println!("Command: {}", command);
                    // println!("Args: {:?}", args);
                }
            }

            // Handle the case where there was an error reading a line
            Some(Err(e)) => {
                eprintln!("Error reading line: {}", e);
            }

            // Handle the case where there are no more lines to read
            None => {
                break; // EOF reached   
            }
        }
    }
}


use std::io::{self, BufRead, BufReader, Stdout, Write, Stdin};
use std::process::Command;
use std::env;
use colored::*;


// Expand environment variables in the command line
// This function checks if a token starts with '$' and replaces it with the corresponding environment variable value.
// If the variable is not found, it returns an empty string.
fn expand_variables(token: &str) -> String {

    // Check if the token starts with '$'
    if token.starts_with('$') {

        // If it does, we remove the '$' and get the variable name
        let var = &token[1..];

        // Check if the variable name is empty
        if var.is_empty() {
            return token.to_string(); // Return the original token if the variable name is empty
        }

        // Get the value of the environment variable
        env::var(var).unwrap_or_else(|_| String::new())

    } else {

        // If the token does not start with '$', we return it as is
        token.to_string()
    }
}

/// Splits a command line into tokens, handling simple whitespace separation.
fn parse_command(line: &str) -> Vec<String> {
    line
        .split_whitespace() // Split by whitespace
        .map(expand_variables) // Convert to String
        .collect() // Collect into a Vec<String>
}

fn pwd() -> String {
    // Get the current working directory
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| String::from("unknown"))
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

    let mut prompt:&str = "lordraleigh>"; // Define the prompt string

    let mut directory:String = pwd(); // Get the current working directory

    loop{

        // Print the prompt
        // The prompt is printed in yellow with a black background.
        print!("{1}{}{}{} ", directory.black().on_yellow(), " ".on_yellow(), "".yellow()); // Prompt the user for input

        
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
                    continue; // Skip empty input
                }

                // Split the input into command and arguments
                // The parse_command function handles simple whitespace separation.
                let parts = parse_command(input);

                let command = &parts[0];
                let args = &parts[1..];
                

                match command.as_str() {


                    // If the input is "exit", we break out of the loop to terminate the program.
                    "exit" => break,

                    "pwd" => {
                        // If the input is "pwd", we print the current working directory.
                        // The pwd() function returns the current working directory as a String.
                        println!("{}", directory);
                        continue;
                    }

                    // If the imput is "cd", we change the current directory.
                    // The std::env::set_current_dir function is used to change the current working directory.
                    // We use the first argument as the new directory.
                    "cd" => {
                        let new_dir = args.get(0).map(|s| s.as_str()).unwrap_or("/");
                        if let Err(e) = std::env::set_current_dir(new_dir) {
                            eprintln!("cd error: {}", e);
                        }
                        directory = pwd(); // Update the current directory
                        
                        continue;
                    }

                    // env is a special command that prints the environment variables.
                    // The env::vars() function returns an iterator over the environment variables.
                    // We use the colored crate to print the variables in a nice format.
                    "env" => {

                        for (key, value) in env::vars() {
                            println!("{} = {}", key.green(), value);
                        }
                        continue;
                    }


                    // If the input is not empty and not a special command, we proceed to execute it.
                    // We split the input into command and arguments using the parse_command function.
                    // The parse_command function handles simple whitespace separation.
                    // We then create a new Command using the command and arguments.
                    // The Command struct is used to spawn child processes.
                    _ => { 

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
                                    Ok(_status) => continue, //println!("Process exited with: {}", status),
        
                                    // If there was an error waiting for the command, we print an error message.
                                    Err(e) => eprintln!("{} {}", "Failed to wait on child process:".red().bold(), e.to_string().red()),
                                }
                            }
        
                            // If there was an error executing the command, we print an error message.
                            Err(e) => {
                                eprintln!("{} {}: {}", "Failed to execute".red(), command.red(), e.to_string().red().bold());
                            }
                        }

                    }
                }
                            
                // Print the command and arguments for debugging purposes
                // This is useful for understanding what command was executed.
                // println!("Command: {}", command);
                // println!("Args: {:?}", args);
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


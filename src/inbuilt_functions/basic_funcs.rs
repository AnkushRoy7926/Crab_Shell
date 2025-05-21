use std::env;
use colored::*;

pub fn pwd() -> String {
    // Get the current working directory
    std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| String::from("unknown"))
}

// If the imput is "cd", we change the current directory.
// The std::env::set_current_dir function is used to change the current working directory.
// We use the first argument as the new directory.
pub fn cd(args: &[String], directory: &mut String) {
    let new_dir = args.get(0).map(|s| s.as_str()).unwrap_or("/");
    if let Err(e) = std::env::set_current_dir(new_dir) {
        eprintln!("cd error: {}", e);
    }

    *directory = pwd(); // Update the current directory

}

// env is a special command that prints the environment variables.
// The env::vars() function returns an iterator over the environment variables.
// We use the colored crate to print the variables in a nice format.
pub fn env() {

    for (key, value) in env::vars() {
        println!("{} = {}", key.green(), value);
    }

}